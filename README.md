# LifeFarm

LifeFarm은 매일의 삶을 살아가는 이들이 인생의 방향을 놓치지 않게 도와주는 서비스이다.

인생을 하나의 농사짓기에 빗대어 시각화함으로써
삶이 어떻게 흘러가는지에 대한 지속적인 피드백 제공을 목표로 한다.

서비스에서는

1. 자신이 살아가고자 하는 방향에 대해 명문화하고 (Farm Blueprint)
2. 하루 하루의 삶을 기록하며 (Daily Farming System)
3. 변화해가는 농장을 보여주며 피드백을 제공한다. (Farm Visualizer)

## Code Name

whistle

- 이무진-'휘파람'

## Language

Rust

## Setup (Mac)

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source $HOME/.cargo/env

rustc --version

cargo new whistle

cargo build

cargo run

# Diesel Migration
diesel migration generate {migration_name}

-- setup up.sql, down.sql --

diesel migration run

diesel print-schema > src/schema.rs

if necessary) diesel migration revert


## Test

cargo test

cargo test -- --nocapture (test 시에도 println!() 출력)

## Architecture

Lifefarm의 모듈은 크게 세 가지로 나뉜다.

현재(24.08.23)는 프로젝트의 극초기 단계이므로 rough한 컨셉만 가지고 간다.

(\* 추후 개발이 진행되면서 지속적으로 구체화할 계획)

1. Farm Blueprint

   - 살아가고자 하는 방향을 명문화한다.
   - 투자해야 하는 시간, 예상 기간 등을 고려하여 전체 Farm에서 어느 정도의 영역을 차지할지 지정한다.

2. Daily Farming System

   - Farm Blueprint에서 지정한 주제와 관련하여 매일 매일 진행한 일들을 기록한다.
   - 목표치와 진행치를 기록한다.
   - 사진, 글 등 인증 자료를 함께 저장한다.

3. Farm Visualizer
   - Farm Blueprint 와 Daily Farming System 에 기록된 정보들을 시각화하여 사용자에게 보여준다.
   - Farm Blueprint
     - 전체 Farm 영역과 해당 영역 내에서 각 Blueprint별로 지정된 Blueprint Farm 영역을 시각화한다.
   - Daily Farm System
     - 하나의 Blueprint Farm 영역 내에서 Daily로 심는 씨앗들을 시각화한다.
     - 첫 시도는 씨앗으로 표현되고, 이후의 시도들은 물/햇빛 등으로 표현된다.
     - 씨앗이 어느정도 자라서 무형적인 자산 (예를 들면 자신감, 자부심 등) 이 생겼다고 판단된다면 그 근거를 특정 글자수 이상으로 입력받아 새싹으로 성장한다.
     - 무형적인 자산을 넘어 작더라도 유형의 결과 (점수 상승, 자격증 취득, 칭찬 등)를 얻게 된다면 새싹에서 풀잎으로 성장한다.
     - 이후 원하는 목표에 절반정도 다가선 시점에서 풀잎이 나무(혹은 곡식 등)로 성장한다.
     - 최종적으로 원하는 목표에 도달한 경우 나무(혹은 곡식 등)에 열매가 맺힌다.
