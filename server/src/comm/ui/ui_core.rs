use chrono::TimeZone;
use chrono::Utc;
use log::warn;

use axum::http::StatusCode;

use crate::enums::AsstIdStatus;
use crate::util::{rest::RestResult, machine_id};
use crate::data::repository::{asst_user_info_repo, asst_system_info_repo, asst_mgmt_repo, agent_repo, intg_audit_repo};
use crate::data::entity::{asst_mgmt_entity::NewAsstMgmtEntity, asst_user_info_entity::NewAsstUserInfoEntity, asst_system_info_entity::NewAsstSystemInfoEntity};

use super::dto::req::*;
use super::dto::res::*;

pub fn get_asst_id() -> (AsstIdStatus, String, String) {
    match asst_user_info_repo::get_latest() {
        Some(user_info) => {
            if 0 == user_info.asst_id_status {
                (
                    AsstIdStatus::NotRegistered,
                    "".to_string(),
                    "".to_string()
                )
            } else if 1 == user_info.asst_id_status {
                let datetime = Utc.timestamp_opt(user_info.asst_id_reg_time, 0).single()
                                .expect("invalid timestamp. failed to convert ASST_ID registration time.");

                (
                    AsstIdStatus::NotRegistered,
                    "".to_string(),
                    datetime.format("%Y/%m/%d %H:%M:%S").to_string()
                )
            } else if 2 == user_info.asst_id_status {
                let datetime = Utc.timestamp_opt(user_info.asst_id_reg_time, 0).single()
                    .expect("invalid timestamp. failed to convert ASST_ID registration time.");

                (
                    AsstIdStatus::Registered,
                    "".to_string(),
                    datetime.format("%Y/%m/%d %H:%M:%S").to_string()
                )
            } else {
                let datetime = Utc.timestamp_opt(user_info.asst_id_reg_time, 0).single()
                    .expect("invalid timestamp. failed to convert ASST_ID registration time.");

                (
                    AsstIdStatus::Issued,
                    user_info.asst_id.unwrap_or_default(),
                    datetime.format("%Y/%m/%d %H:%M:%S").to_string()
                )
            }
        },
        None => {
            (
                AsstIdStatus::NotRegistered,
                "".to_string(),
                "".to_string()
            )
        }
    }
}

pub fn get_system_name_list() -> GetSystemNameListResponse {
    let system_names = asst_system_info_repo::mockup_system_list_from_coontec()
            .iter()
            .map(|system| GetSystemNameResponse {
                sys_id: system.system_id.clone(),
                sys_name: system.system_nm.clone().unwrap_or_default()
            }).collect();

    GetSystemNameListResponse {
        systems: system_names
    }
}

pub fn get_asst() -> Result<GetAsstResponse, RestResult> {
    let cur_asst_mgmt = asst_mgmt_repo::get_latest();
    if true == cur_asst_mgmt.is_none() {
        return Err(RestResult {
            code: StatusCode::NOT_FOUND,
            err_msg: "asset management not exists.".to_string()
        });
    }

    let cur_user_info = asst_user_info_repo::get_latest();
    if true == cur_user_info.is_none() {
        return Err(RestResult {
            code: StatusCode::NOT_FOUND,
            err_msg: "user information not exists.".to_string()
        });
    }

    let cur_system_info =
        asst_system_info_repo::mockup_selected_system_from_user(cur_user_info.clone().unwrap().system_id);
    if true == cur_system_info.is_none() {
        return Err(RestResult {
            code: StatusCode::NOT_FOUND,
            err_msg: "system information not exists.".to_string()
        });
    }

    let asst_mgmt = cur_asst_mgmt.unwrap();
    let user_info = cur_user_info.unwrap();
    let system_info = cur_system_info.unwrap();

    Ok(GetAsstResponse {
        asset_management: GetAsstMgmtResponse {
            system_id: asst_mgmt.system_id,
            mdcd: asst_mgmt.mdcd.unwrap_or_default(),
            asst_user_uc: asst_mgmt.asst_user_uc.unwrap_or_default(),
            rmucd: asst_mgmt.rmucd.unwrap_or_default(),
            info_rsrc_clsfct_cd: asst_mgmt.info_rsrc_clsfct_cd.unwrap_or_default(),
            eqp_dvs_cd: asst_mgmt.eqp_dvs_cd.unwrap_or_default(),
            asst_sn: asst_mgmt.asst_sn.unwrap_or_default(),
            asst_user_id: user_info.asst_id.unwrap_or_default(),
            scrt_grd_cd: asst_mgmt.scrt_grd_cd.unwrap_or_default(),
            dvlp_entprz_nm: asst_mgmt.dvlp_entprz_nm.unwrap_or_default(),
            mtnc_rpr_ctrt_yn: asst_mgmt.mtnc_rpr_ctrt_yn.unwrap_or_default() == 1,
            mtnc_rpr_entprz_nm: asst_mgmt.mtnc_rpr_entprz_nm.unwrap_or_default(),
            asst_dtl_loctn_ctnt: asst_mgmt.asst_dtl_loctn_ctnt.unwrap_or_default(),
            eqp_knd_nm: asst_mgmt.eqp_knd_nm.unwrap_or_default(),
            eqp_mfbiz_nm: asst_mgmt.eqp_mfbiz_nm.unwrap_or_default(),
            eqp_model_nm: asst_mgmt.eqp_model_nm.unwrap_or_default(),
            eqp_natlt_ctnt: asst_mgmt.eqp_natlt_ctnt.unwrap_or_default(),
            eqp_dvc_dtl_ctnt: asst_mgmt.eqp_dvc_dtl_ctnt.unwrap_or_default(),
            etc_ctnt: asst_mgmt.etc_ctnt.unwrap_or_default()
        },
        system_info: GetSystemInfoResponse {
            system_nm: system_info.system_nm.unwrap_or_default(),
            mdcd: system_info.mdcd.unwrap_or_default(),
            asgmt_uc: system_info.asgmt_uc.unwrap_or_default(),
            mn_nd_is_yn: system_info.mn_nd_is_yn.unwrap_or_default() == 1,
            system_prtctn_grd_ctnt: system_info.system_prtctn_grd_ctnt.unwrap_or_default(),
            asst_sysop_uc: system_info.asst_sysop_uc.unwrap_or_default(),
            rmucd: system_info.rmucd.unwrap_or_default(),
            info_rsrc_clsfct_cd: system_info.info_rsrc_clsfct_cd.unwrap_or_default(),
            url: system_info.url.unwrap_or_default(),
            introdt: system_info.introdt.unwrap_or_default(),
            prj_nm: system_info.prj_nm.unwrap_or_default(),
            scrt_grd_cd: system_info.scrt_grd_cd.unwrap_or_default(),
            dvlp_entprz_nm: system_info.dvlp_entprz_nm.unwrap_or_default(),
            dlvgds_entprz_nm: system_info.dlvgds_entprz_nm.unwrap_or_default(),
            mtnc_rpr_ctrt_yn: system_info.mtnc_rpr_ctrt_yn.unwrap_or_default() == 1,
            mtnc_rpr_entprz_nm: system_info.mtnc_rpr_entprz_nm.unwrap_or_default(),
            etc_info_ctnt: system_info.etc_info_ctnt.unwrap_or_default()
        },
        user_info: GetUserInfoResponse {
            rnkdvcd: user_info.rnkdvcd,
            rnkcd: user_info.rnkcd,
            asst_user_nm: user_info.asst_user_nm,
            asst_sysop_uc: user_info.asst_sysop_uc,
            asst_user_telno: user_info.asst_user_telno,
            relay_server_ip: user_info.bridge_server_ip,
            sys_id: user_info.system_id
        }
    })
}

pub fn update_user(info: PutUserRequest)-> Result<(), RestResult> {
    match asst_user_info_repo::get_latest() {
        Some(mut user_info) => {
            //// TODO: extract validation
            if user_info.asst_user_ecryptpw != info.org_user_ecryptpw {
                return Err(RestResult {
                    code: StatusCode::FORBIDDEN,
                    err_msg: "password validation failed.".to_string()
                });
            }

            if info.asst_user_ecryptpw != info.asst_user_ecryptpw_confirm {
                return Err(RestResult {
                    code: StatusCode::BAD_REQUEST,
                    err_msg: "new password confirm not matched.".to_string()
                });
            }
            ///////

            user_info.rnkdvcd = info.rnkdvcd;
            user_info.rnkcd = info.rnkcd;
            user_info.asst_user_nm = info.asst_user_nm;
            user_info.asst_user_ecryptpw = info.asst_user_ecryptpw;
            user_info.asst_sysop_uc = info.asst_sysop_uc;
            user_info.asst_user_telno = info.asst_user_telno;
            user_info.bridge_server_ip = info.relay_server_ip;
            user_info.system_id = info.sys_id;

            match asst_user_info_repo::update(&user_info) {
                Ok(_) => Ok(()),
                Err(err_msg) => Err(RestResult {
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                    err_msg
                })
            }
        },
        None => Err(RestResult {
            code: StatusCode::NOT_FOUND,
            err_msg: "user information not exist.".to_string()
        })
    }
}

pub fn update_asst_mgmt(info: PutAsstMgmtRequest) -> Result<(), RestResult> {
    match asst_user_info_repo::get_latest() {
        Some(user_info) => {
            //// TODO: extract validation
            if user_info.asst_user_ecryptpw != info.org_user_ecryptpw {
                return Err(RestResult {
                    code: StatusCode::FORBIDDEN,
                    err_msg: "password validation failed.".to_string()
                });
            }
            ///////

            let new_mtnc_rpr_ctrt_yn = if true == info.mtnc_rpr_ctrt_yn { 1 } else { 0 };
            //
            // TODO: get latest 해야 함, ASST_MGMT 는 하나만 존재
            match asst_mgmt_repo::get(&user_info.system_id) {
                Some(mut mgmt_entity) => {
                    mgmt_entity.system_id = info.system_id;
                    mgmt_entity.mdcd = Some(info.mdcd);
                    mgmt_entity.asst_user_uc = Some(info.asst_user_uc);
                    mgmt_entity.rmucd = Some(info.rmucd);
                    mgmt_entity.info_rsrc_clsfct_cd = Some(info.info_rsrc_clsfct_cd);
                    mgmt_entity.eqp_dvs_cd = Some(info.eqp_dvs_cd);
                    mgmt_entity.asst_sn = Some(info.asst_sn);
                    mgmt_entity.scrt_grd_cd = Some(info.scrt_grd_cd);
                    mgmt_entity.dvlp_entprz_nm = Some(info.dvlp_entprz_nm);
                    mgmt_entity.mtnc_rpr_ctrt_yn = Some(new_mtnc_rpr_ctrt_yn);
                    mgmt_entity.mtnc_rpr_entprz_nm = Some(info.mtnc_rpr_entprz_nm);
                    mgmt_entity.asst_dtl_loctn_ctnt = Some(info.asst_dtl_loctn_ctnt);
                    mgmt_entity.eqp_knd_nm = Some(info.eqp_knd_nm);
                    mgmt_entity.eqp_mfbiz_nm = Some(info.eqp_mfbiz_nm);
                    mgmt_entity.eqp_model_nm = Some(info.eqp_model_nm);
                    mgmt_entity.eqp_natlt_ctnt = Some(info.eqp_natlt_ctnt);
                    mgmt_entity.eqp_dvc_dtl_ctnt = Some(info.eqp_dvc_dtl_ctnt);
                    mgmt_entity.etc_ctnt = Some(info.etc_ctnt);

                    match asst_mgmt_repo::update(&mgmt_entity) {
                        Ok(_) => Ok(()),
                        Err(err_msg) =>{
                            warn!("{}", err_msg);

                            Err(RestResult {
                                code: StatusCode::INTERNAL_SERVER_ERROR,
                                err_msg: "failed to update asset management.".to_string()
                            })
                        }
                    }
                },
                None => Err(RestResult {
                    code: StatusCode::NOT_FOUND,
                    err_msg: "asset management not exist.".to_string()
                })
            }
        },
        None => Err(RestResult {
            code: StatusCode::NOT_FOUND,
            err_msg: "user information not exist.".to_string()
        })
    }
}

pub fn get_agent() -> GetAgentListReponse {
    GetAgentListReponse {
        intg_agent_status: 0, // dummy for test
        agents: agent_repo::get_all()
                    .iter()
                    .map(|ag| GetAgentResponse {
                        name: ag.name.clone(),
                        agent_id: ag.agent_id.clone(),
                        agent_status: ag.status.unwrap_or_default(),
                        version: ag.version.clone().unwrap_or_default(),
                        agent_update_time: Utc.timestamp_opt(ag.updated_time, 0).single()
                            .expect("invalid timestamp. failed to convert ASST_ID registration time.")
                            .format("%Y/%m/%d %H:%M:%S").to_string(),
                        last_inspect_time: "2000/01/01 01:01:01".to_string(), // dummy for test
                        last_inspect_status: 0, // dummy for test
                        next_inspect_time: "2000/01/02 01:01:01".to_string() // dummy for test
                    })
                    .collect()
    }
}

pub fn post_asset(info: PostAsstRequest) -> Result<(), RestResult> {

    match asst_mgmt_repo::add(&NewAsstMgmtEntity {
        system_id: info.asset_management.system_id,
        mdcd: Some(info.asset_management.mdcd),
        asst_user_uc: Some(info.asset_management.asst_user_uc),
        rmucd: Some(info.asset_management.rmucd),
        info_rsrc_clsfct_cd: Some(info.asset_management.info_rsrc_clsfct_cd),
        eqp_dvs_cd: Some(info.asset_management.eqp_dvs_cd),
        asst_sn: Some(info.asset_management.asst_sn),
        scrt_grd_cd: Some(info.asset_management.scrt_grd_cd),
        dvlp_entprz_nm: Some(info.asset_management.dvlp_entprz_nm),
        mtnc_rpr_ctrt_yn: if true == info.asset_management.mtnc_rpr_ctrt_yn { Some(1) } else { Some(0) },
        mtnc_rpr_entprz_nm: Some(info.asset_management.mtnc_rpr_entprz_nm),
        asst_dtl_loctn_ctnt: Some(info.asset_management.asst_dtl_loctn_ctnt),
        eqp_knd_nm: Some(info.asset_management.eqp_knd_nm),
        eqp_mfbiz_nm: Some(info.asset_management.eqp_mfbiz_nm),
        eqp_model_nm: Some(info.asset_management.eqp_model_nm),
        eqp_natlt_ctnt: Some(info.asset_management.eqp_natlt_ctnt),
        eqp_dvc_dtl_ctnt: Some(info.asset_management.eqp_dvc_dtl_ctnt),
        etc_ctnt: Some(info.asset_management.etc_ctnt)
    }) {
        Ok(_) => {},
        Err(err_msg) => {
            warn!("{}", err_msg);

            return Err(RestResult {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                err_msg: format!("failed to add asset management. (Error: {})", err_msg)
            });
        }
    };

    let new_system_id = info.user_info.sys_id;

    match asst_user_info_repo::add(&NewAsstUserInfoEntity{
        machine_id: machine_id::gen_machine_id(),
        system_id: new_system_id.clone(),
        asst_id: None,
        rnkdvcd: info.user_info.rnkdvcd,
        rnkcd: info.user_info.rnkcd,
        asst_user_nm: info.user_info.asst_user_nm,
        asst_user_ecryptpw: info.user_info.asst_user_ecryptpw,
        asst_sysop_uc: info.user_info.asst_sysop_uc,
        asst_user_telno: info.user_info.asst_user_telno,
        bridge_server_ip: info.user_info.relay_server_ip,
        asst_id_status: 0,
        asst_id_reg_time: 0
    }) {
        Ok(_) => {},
        Err(err_msg) => {
            warn!("{}", err_msg);

            return Err(RestResult {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                err_msg: format!("failed to add asset user information. (Error: {})", err_msg)
            });
        }
    };

    match asst_system_info_repo::mockup_selected_system_from_user(new_system_id.clone()) {
        Some(system_info) => {
            match asst_system_info_repo::add(&NewAsstSystemInfoEntity {
                system_id: system_info.system_id,
                system_nm: system_info.system_nm,
                mdcd: system_info.mdcd,
                asgmt_uc: system_info.asgmt_uc,
                mn_nd_is_yn: system_info.mn_nd_is_yn,
                system_prtctn_grd_ctnt: system_info.system_prtctn_grd_ctnt,
                asst_sysop_uc: system_info.asst_sysop_uc,
                rmucd: system_info.rmucd,
                info_rsrc_clsfct_cd: system_info.info_rsrc_clsfct_cd,
                url: system_info.url,
                introdt: system_info.introdt,
                prj_nm: system_info.prj_nm,
                scrt_grd_cd: system_info.scrt_grd_cd,
                dvlp_entprz_nm: system_info.dvlp_entprz_nm,
                dlvgds_entprz_nm: system_info.dlvgds_entprz_nm,
                mtnc_rpr_ctrt_yn: system_info.mtnc_rpr_ctrt_yn,
                mtnc_rpr_entprz_nm: system_info.mtnc_rpr_entprz_nm,
                etc_info_ctnt: system_info.etc_info_ctnt
            }) {
                Ok(_) => Ok(()),
                Err(err_msg) => {
                    warn!("{}", err_msg);

                    Err(RestResult {
                        code: StatusCode::INTERNAL_SERVER_ERROR,
                        err_msg: format!("failed to add system information. (Error: {})", err_msg)
                    })
                }
            }
        },
        None => {
            let err_msg = format!("system information with id {} not exists.", new_system_id);

            warn!("{}", err_msg);
            Err(RestResult {
                code: StatusCode::BAD_REQUEST,
                err_msg
            })
        }
    }
}

pub fn post_validate(info: PostValidateRequest) -> Result<bool, RestResult> {
    Ok(true)
}

pub fn post_unlock(info: PostUnlockRequest) -> Result<bool, RestResult> {
    Ok(true)
}

pub fn get_intgr_log(info: PostLogListRequest) -> GetLogListResponse {
    GetLogListResponse {
        logs: intg_audit_repo::get_by_limit(info.log_limit)
                .iter()
                .map(|audit| GetLogResponse {
                    level: 0, // TODO: add level in audit data
                    time: Utc.timestamp_opt(audit.created_time, 0).single()
                            .expect("invalid timestamp. failed to convert ASST_ID registration time.")
                            .format("%Y/%m/%d %H:%M:%S").to_string(),
                    msg: audit.msg.clone()
                })
                .collect()
    }
}

// fn gen_tmp_log_list(limit: i32) -> GetLogListResponse {
//     GetLogListResponse {
//         logs: intg_audit_repo::get_all()
//         .order(intg_audit_repo::created_time.desc())
//         .iter()
//         .map(|ad| LogResponse {
//             level: 0, // TODO: add level in audit data
//             time: Utc.timestamp_opt(ad.created_time, 0).single()
//                     .expect("invalid timestamp. failed to convert ASST_ID registration time.")
//                     .format("%Y/%m/%d %H:%M:%S").to_string(),
//             msg: ad.msg.clone()
//         })
//         .collect()
//     }
// }

pub fn get_setting() -> Result<GetSettingResponse, RestResult> {
    Ok(GetSettingResponse{
        cmd_period: 360,
        intgr_update_period: 3600
    })
}

pub fn get_delete() -> Result<bool, RestResult> {
    Ok(true)
}

