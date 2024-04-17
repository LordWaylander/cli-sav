use std::fmt;

#[derive(Debug, Clone)]
pub struct DiskPart {
    pub name: String,
    pub size: String,
    pub mountpoint: String,
    pub uuid: String,
    pub path: String

}

impl fmt::Display for DiskPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name : {}, size : {}G, mountpoint : {}, uuid : {}", self.name, self.get_size("giga-byte"), self.mountpoint, self.uuid)
    }
}

impl DiskPart {
    pub fn get_size(&self, unit: &str) -> u64 {
        match unit {
            "byte" => {
                self.size.parse::<u64>().unwrap()
            }
            "mega-byte" => {
                if self.size.parse::<u64>().unwrap()/1048576 == 0 {
                    self.get_size("byte")
                }else {
                    self.size.parse::<u64>().unwrap()/1048576
                }
            }
            "giga-byte" => {
                if self.size.parse::<u64>().unwrap()/1073741824 == 0 {
                    self.get_size("mega-byte")
                }else {
                    self.size.parse::<u64>().unwrap()/1073741824
                }
                
            }
            _ => {
                todo!("only B and GB available");
            }
        }
    }
}