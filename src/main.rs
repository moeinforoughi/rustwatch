use crate::ServiceStatus::Running;

struct SystemInfo {
    os: &'static str,
    architecture: &'static str,
    hostname: String,
}

struct Service {
    name: String,
    status: ServiceStatus,
}

#[derive(PartialEq)]
enum ServiceStatus {
    Running,
    Stopped(String),
    Starting(u8),
    Unknown,
}

impl Service {
    fn display(&self) {
        match &self.status {
            ServiceStatus::Running => {
                println!("{} is running", self.name);
            }

            ServiceStatus::Stopped(reason) => {
                println!("{} stopped: {}", self.name, reason);
            }

            ServiceStatus::Starting(progress) => {
                println!("{} Starting: {}%", self.name, progress);
            }

            ServiceStatus::Unknown => {
                println!("{} status unknown", self.name);
            }
        }
    }

    fn is_running(&self) -> bool {
    self.status == Running
    }

}

impl SystemInfo {
    fn new() -> Self {
        let os:&'static str = std::env::consts::OS;
        let architecture: &'static str = std::env::consts::ARCH;
        let hostname: String = std::env::var("COMPUTERNAME").unwrap_or(String::from("Unknown"));
        
        Self { os, architecture, hostname }
    }

    fn display(&self) {
        println!("     System Information
        ------------------
        OS: {}
        Architecture: {}
        Hostname: {}"
        , self.os, self.architecture, self.hostname);
    }
}

fn count_errors(content: &str) -> usize {
    content
    .lines()
    .filter(|content| content.contains("ERROR"))
    .count()
}


fn main() {

    let systeminfo = SystemInfo::new();
    systeminfo.display();

    let nginx = Service {
        name: String::from("nginx"),
        status: ServiceStatus::Running,
    };

    let postgres = Service {
        name: String::from("postgres"),
        status: ServiceStatus::Running,
    };

    let api = Service {
        name: String::from("api"),
        status: ServiceStatus::Stopped(String::from("Process not found")),
    };

    let backend = Service {
        name: String::from("backend"),
        status: ServiceStatus::Starting(80),
    };

    let frontend = Service {
        name: String::from("frontend"),
        status: ServiceStatus::Unknown,
    };

    let services: Vec<Service> = vec![
    nginx,
    postgres,
    api,
    backend,
    frontend
    ];

    println!("Service Monitoring
    ------------------");
    for service in &services {
    service.display();
    }
    let running_count = services
    .iter()
    .filter(|service| { service.is_running()})
    .count();

    println!("Total Services: {}", services.len());
    println!("Running Services: {}", running_count);

    println!("logs : ");
    let filedata = std::fs::read_to_string("rustwatch.log");

    match filedata {
        Ok(data) => {
            println!("{}", data);
            println!("Log Analysis \n------------");
            let errors = count_errors(&data);
            println!("Total Errors: {}", errors);
        }
        Err(error) => {
            println!("Failed to read file: {}", error);
        }
    }

}
