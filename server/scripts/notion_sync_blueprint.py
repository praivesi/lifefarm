#!/usr/bin/env python3
"""
Notion 'LifeFarm BluePrint' 페이지의 to_do 항목들을 읽어와
LifeFarm 서버의 POST /front/blpt API로 Blueprint를 생성하는 일회성 스크립트.

사용법:
    cd server
    python3 scripts/notion_sync_blueprint.py [--dry-run]

.env 에서 NOTION_TOKEN 을 읽는다.
"""

import argparse
import json
import os
import ssl
import sys
import urllib.error
import urllib.request
from datetime import datetime, timedelta, timezone
from pathlib import Path

NOTION_PAGE_ID = "3b339935544c80458bbeee8cb7c07fd7"
NOTION_VERSION = "2022-06-28"
SERVER_BASE_URL = "https://localhost:30443"
KST = timezone(timedelta(hours=9))


def load_env(env_path: Path) -> dict:
    env = {}
    for line in env_path.read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        env[key.strip()] = value.strip()
    return env


def notion_get(path: str, token: str) -> dict:
    req = urllib.request.Request(
        f"https://api.notion.com/v1/{path}",
        headers={
            "Authorization": f"Bearer {token}",
            "Notion-Version": NOTION_VERSION,
        },
    )
    with urllib.request.urlopen(req) as resp:
        return json.loads(resp.read())


def block_text(block: dict) -> str:
    t = block["type"]
    content = block.get(t, {})
    rich_text = content.get("rich_text", [])
    return "".join(rt.get("plain_text", "") for rt in rich_text)


def fetch_children(block_id: str, token: str) -> list:
    results = []
    cursor = None
    while True:
        path = f"blocks/{block_id}/children?page_size=100"
        if cursor:
            path += f"&start_cursor={cursor}"
        data = notion_get(path, token)
        results.extend(data.get("results", []))
        if not data.get("has_more"):
            break
        cursor = data.get("next_cursor")
    return results


def fetch_field_value(field_block: dict, token: str) -> str:
    """goal/desc/start_dt/end_dt 라벨 블록의 자식(값)을 읽는다."""
    if not field_block.get("has_children"):
        return ""
    children = fetch_children(field_block["id"], token)
    if not children:
        return ""
    return block_text(children[0]).strip()


def date_to_kst_epoch(date_str: str) -> int:
    """'2026-08-03' -> KST 자정 기준 second epoch timestamp"""
    dt = datetime.strptime(date_str, "%Y-%m-%d").replace(tzinfo=KST)
    return int(dt.timestamp())


def collect_blueprints(page_id: str, token: str) -> list:
    top_blocks = fetch_children(page_id, token)
    blueprints = []
    for block in top_blocks:
        if block["type"] != "to_do":
            continue
        title = block_text(block).strip()
        fields = {}
        for field_block in fetch_children(block["id"], token):
            if field_block["type"] != "bulleted_list_item":
                continue
            label = block_text(field_block).strip()
            if label in ("goal", "desc", "start_dt", "end_dt"):
                fields[label] = fetch_field_value(field_block, token)

        missing = [k for k in ("goal", "desc", "start_dt", "end_dt") if not fields.get(k)]
        if missing:
            print(f"  [skip] {title!r}: 누락된 필드 {missing}", file=sys.stderr)
            continue

        blueprints.append({
            "title": title,
            "goal": fields["goal"],
            "desc": fields["desc"],
            "start_dt": date_to_kst_epoch(fields["start_dt"]),
            "end_dt": date_to_kst_epoch(fields["end_dt"]),
        })
    return blueprints


def post_blueprint(bp: dict, insecure_ctx: ssl.SSLContext) -> dict:
    payload = json.dumps({
        "goal": bp["goal"],
        "desc": bp["desc"],
        "start_dt": bp["start_dt"],
        "end_dt": bp["end_dt"],
    }).encode()

    req = urllib.request.Request(
        f"{SERVER_BASE_URL}/front/blpt",
        data=payload,
        method="POST",
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, context=insecure_ctx) as resp:
        return json.loads(resp.read())


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--dry-run", action="store_true", help="Notion만 읽고 서버로 전송하지 않음")
    args = parser.parse_args()

    server_dir = Path(__file__).resolve().parent.parent
    env = load_env(server_dir / ".env")
    token = env.get("NOTION_TOKEN")
    if not token:
        print("server/.env 에 NOTION_TOKEN 이 없습니다.", file=sys.stderr)
        sys.exit(1)

    print("Notion에서 Blueprint 항목 읽는 중...")
    blueprints = collect_blueprints(NOTION_PAGE_ID, token)
    print(f"{len(blueprints)}개 항목 발견\n")

    for bp in blueprints:
        print(f"- {bp['title']}")
        print(f"    goal     : {bp['goal']}")
        print(f"    desc     : {bp['desc']}")
        print(f"    start_dt : {bp['start_dt']}")
        print(f"    end_dt   : {bp['end_dt']}")

    if args.dry_run:
        print("\n--dry-run 모드: 서버로 전송하지 않았습니다.")
        return

    # 로컬 서버가 자체 서명 인증서를 쓰므로 검증을 생략한다.
    insecure_ctx = ssl.create_default_context()
    insecure_ctx.check_hostname = False
    insecure_ctx.verify_mode = ssl.CERT_NONE

    print(f"\n{SERVER_BASE_URL}/front/blpt 로 전송 중...")
    created, failed = 0, 0
    for bp in blueprints:
        try:
            result = post_blueprint(bp, insecure_ctx)
            print(f"  [ok] {bp['title']} -> id={result.get('id')}")
            created += 1
        except urllib.error.URLError as e:
            print(f"  [fail] {bp['title']}: {e}", file=sys.stderr)
            failed += 1

    print(f"\n완료: {created}개 생성, {failed}개 실패")


if __name__ == "__main__":
    main()
