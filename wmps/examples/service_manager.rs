use std::ffi::OsString;
use std::env;
use windows_service::{
    service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType, ServiceState},
    service_manager::{ServiceManager, ServiceManagerAccess},
};
use std::{thread, time::Duration};

/// 服务管理入口函数
#[cfg(windows)]
fn main() -> windows_service::Result<()>{
    let args: Vec<String> = env::args().collect();

    // 获取操作类型
    let oper_type = if args.len() > 1 {
        &args[1]
    }
    else {
        "install"
    };

    // 服务名称
    let service_name = if args.len() > 2 {
        &args[2]
    }
    else{
        "xservice"
    };

    // 二进制文件名称
    let service_binary_name = if args.len() > 3 {
        &args[3]
    }
    else {
        "xservice.exe"
    };

    // 服务描述
    let service_descript = if args.len() > 4 {
        &args[4]
    }
    else {
        "xservice description"
    };

    // 进行分类操作分发
    match oper_type {
        "install" => {
            install(service_name, service_binary_name, service_descript)
        },
        _ => {
            operate(oper_type, service_name)
        }
    }
}


#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}

/// 安装服务
fn install(service_name: &str, service_binary_name: &str, service_descript: &str)  -> windows_service::Result<()>{
    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    // println!("service name {}, service path: {}, service description: {}", service_name, service_binary_name, service_descript);
    // return;

    // This example installs the service defined in `examples/ping_service.rs`.
    // In the real world code you would set the executable path to point to your own binary
    // that implements windows service.
    let service_binary_path = ::std::env::current_exe()
        .unwrap()
        .with_file_name(service_binary_name);

    let service_info = ServiceInfo {
        name: OsString::from(service_name),
        display_name: OsString::from(service_descript),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::OnDemand,
        error_control: ServiceErrorControl::Normal,
        executable_path: service_binary_path,
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: None, // run as System
        account_password: None,
    };
    let _service = service_manager.create_service(service_info, ServiceAccess::empty())?;

    println!("service {} install successfully!", service_name);

    Ok(())
}


/// 服务的其他操作
fn operate(oper_type: &str, service_name: &str)-> windows_service::Result<()>{
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;
    let service = service_manager.open_service(service_name, service_access)?;

    let service_status = service.query_status()?;

    match oper_type {
        "stop" => {
            if service_status.current_state != ServiceState::Stopped {
                service.stop()?;
                // Wait for service to stop
                thread::sleep(Duration::from_secs(1));
            }
        },
        "start" => {
            if service_status.current_state == ServiceState::Stopped {
                service.start(&[service_name])?;
            }
        },
        "restart" => {
            if service_status.current_state != ServiceState::Stopped {
                service.stop()?;
                // Wait for service to stop
                thread::sleep(Duration::from_secs(1));
            }
            service.start(&[service_name])?;
        },
        "uninstall" => {
             if service_status.current_state != ServiceState::Stopped {
                service.stop()?;
                // Wait for service to stop
                thread::sleep(Duration::from_secs(1));
            }
            service.delete()?;
        },
        _ => {
            println!("not found operate type!");
        }
    }
    println!("service {} operate {} successfully !", service_name, oper_type);

    Ok(())
}
