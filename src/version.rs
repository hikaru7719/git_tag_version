use std::cmp::Ordering;

#[derive(Eq)]
pub struct SemVer {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

#[derive(Debug)]
pub enum VersioningErr {
    NotSemVer,
}

impl SemVer {
    pub fn new(version: &mut String) -> Result<SemVer, VersioningErr> {
        if version.remove(0) != 'v' {
            return Err(VersioningErr::NotSemVer);
        }
        let v: Vec<&str> = version.split(".").collect();
        if v.len() != 3 {
            return Err(VersioningErr::NotSemVer);
        }
        return Ok(SemVer {
            major: v[0].parse().unwrap(),
            minor: v[1].parse().unwrap(),
            patch: v[2].parse().unwrap(),
        });
    }
    pub fn to_string(&self) -> String {
        return format!("v{}.{}.{}", self.major, self.minor, self.patch);
    }
}

impl Ord for SemVer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch))
    }
}

impl PartialOrd for SemVer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SemVer {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == self.minor && self.patch == self.patch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_semantic_version() {
        struct TestDate {
            version: String,
            err: bool,
            expect: SemVer,
        }
        let mut testcase: Vec<&mut TestDate> = Vec::new();
        let mut data1 = TestDate {
            version: String::from("v1.0.0"),
            err: false,
            expect: SemVer {
                major: 1,
                minor: 0,
                patch: 0,
            },
        };
        let mut data2 = TestDate {
            version: String::from("1.0.0"),
            err: true,
            expect: SemVer {
                major: 0,
                minor: 0,
                patch: 0,
            },
        };
        testcase.push(&mut data1);
        testcase.push(&mut data2);

        for v in testcase {
            match SemVer::new(&mut v.version) {
                Ok(acutual) => {
                    assert_eq!(v.err, false);
                    assert_eq!(acutual.major, v.expect.major);
                    assert_eq!(acutual.minor, v.expect.minor);
                    assert_eq!(acutual.patch, v.expect.patch);
                }
                Err(_) => {
                    assert_eq!(v.err, true);
                }
            }
        }
    }

    #[test]
    fn test_sort() {
        let mut vec: Vec<SemVer> = Vec::new();
        let version1 = SemVer::new(&mut String::from("v1.0.0")).unwrap();
        let version2 = SemVer::new(&mut String::from("v1.0.1")).unwrap();
        let version3 = SemVer::new(&mut String::from("v1.0.0")).unwrap();
        let version4 = SemVer::new(&mut String::from("v1.1.0")).unwrap();
        let version5 = SemVer::new(&mut String::from("v1.1.1")).unwrap();
        let version6 = SemVer::new(&mut String::from("v2.0.0")).unwrap();
        vec.push(version1);
        vec.push(version2);
        vec.push(version3);
        vec.push(version4);
        vec.push(version5);
        vec.push(version6);
        vec.sort();
        assert_eq!(vec[0].to_string(), "v1.0.0");
        assert_eq!(vec[1].to_string(), "v1.0.0");
        assert_eq!(vec[2].to_string(), "v1.0.1");
        assert_eq!(vec[3].to_string(), "v1.1.0");
        assert_eq!(vec[4].to_string(), "v1.1.1");
        assert_eq!(vec[5].to_string(), "v2.0.0");
    }
}
