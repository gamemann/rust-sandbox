use anyhow::Result;

use serde::{Deserialize, Serialize};

/* --- DATA 1 ---  */
/* ---------------- */
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EnumDataOne {
    pub f1: String,
    pub f2: i32,
    pub f3: bool
}

/* --- DATA 2 --- */
/* ---------------- */
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct EnumDataTwo {
    pub f1: String,
    pub f2: i32,
    pub f3: EnumDataTwoV1
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum EnumDataTwoV1 {
    V1 { f1: String, f2: u32, f3: bool },
    V2(EnumDataTwoV2),
    V3 { f7: String, f8: i32, f9: bool },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct EnumDataTwoV2 {
    pub f4: String,
    pub f5: i32,
    pub f6: bool
}

/* --- DATA 3 --- */
/* ---------------- */
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EnumDataThree {
    V1(EnumDataThreeV1),
    V2(EnumDataThreeV2)
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EnumDataThreeV1 {
    pub f1: String,
    pub f2: u32,
    pub f3: bool
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EnumDataThreeV2 {
    pub f4: String,
    pub f5: i32,
    pub f6: bool
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum EnumData {
    #[serde(rename = "one")]
    One(EnumDataOne),
    #[serde(rename = "two")]
    Two(EnumDataTwo),
    #[serde(rename = "three")]
    Three(EnumDataThree)
}

fn main() -> Result<()> {
    // Test with basic data structure.
    let d1 = EnumData::One(EnumDataOne {
        f1: "My String".to_string(),
        f2: 42,
        f3: true
    });

    println!("Data One: {:?}", d1);

    assert_eq!(d1, EnumData::One(EnumDataOne {
        f1: "My String".to_string(),
        f2: 42,
        f3: true
    }));

    // Now serialize and deserialize it.
    let d1_se = serde_json::to_string(&d1)?;
    println!("Data One Se: {}", d1_se);

    let d1_de: EnumData = serde_json::from_str(&d1_se)?;

    println!("Data One De: {:?}", d1_de);

    assert_eq!(d1_de, EnumData::One(EnumDataOne {
        f1: "My String".to_string(),
        f2: 42,
        f3: true
    }));

    println!();
    println!();

    // Construct data two with V1 first.
    let mut d2_f3 = EnumDataTwoV1::V1{
        f1: "Inner String".to_string(), f2: 200, f3: true 
    };

    let mut d2 = EnumData::Two(EnumDataTwo {
        f1: "Another String".to_string(),
        f2: 100,
        f3: d2_f3.clone()
    });
    
    println!("Data Two: {:?}", d2);

    assert_eq!(d2, EnumData::Two(EnumDataTwo {
        f1: "Another String".to_string(),
        f2: 100,
        f3: d2_f3.clone()
    }));

    // Now serialize and deserialize it.
    let d2_se = serde_json::to_string(&d2)?;
    println!("Data Two Se: {}", d2_se);

    let d2_de: EnumData = serde_json::from_str(&d2_se)?;

     println!("Data Two De: {:?}", d2_de);

    assert_eq!(d2_de, EnumData::Two(EnumDataTwo {
        f1: "Another String".to_string(),
        f2: 100,
        f3: d2_f3.clone()
    }));

    // Now test with V2
    d2_f3 = EnumDataTwoV1::V2(EnumDataTwoV2 {
        f4: "Inner String V2".to_string(),
        f5: 300,
        f6: false
    });

    d2 = EnumData::Two(EnumDataTwo {
        f1: "Another String".to_string(),
        f2: 100,
        f3: d2_f3.clone()
    });

    println!("Data Two V2: {:?}", d2);

    assert_eq!(d2, EnumData::Two(EnumDataTwo {
        f1: "Another String".to_string(),
        f2: 100,
        f3: EnumDataTwoV1::V2(EnumDataTwoV2 {
            f4: "Inner String V2".to_string(),
            f5: 300,
            f6: false
        })
    }));

    // Now serialize and deserialize it.
    let d2_se = serde_json::to_string(&d2)?;
    println!("Data Two V2 Se: {}", d2_se);

    let d2_de: EnumData = serde_json::from_str(&d2_se)?;

     println!("Data Two V2 De: {:?}", d2_de);

    assert_eq!(d2_de, EnumData::Two(EnumDataTwo {
        f1: "Another String".to_string(),
        f2: 100,
        f3: EnumDataTwoV1::V2(EnumDataTwoV2 {
            f4: "Inner String V2".to_string(),
            f5: 300,
            f6: false
        })
    }));

    // Now test with V3.
    d2_f3 = EnumDataTwoV1::V3 {
        f7: "Inner String V3".to_string(),
        f8: 400,
        f9: true
    };

    d2 = EnumData::Two(EnumDataTwo {
        f1: "Another String".to_string(),
        f2: 100,
        f3: d2_f3.clone()
    });

    println!("Data Two V3: {:?}", d2);

    assert_eq!(d2, EnumData::Two(EnumDataTwo {
        f1: "Another String".to_string(),
        f2: 100,
        f3: EnumDataTwoV1::V3 {
            f7: "Inner String V3".to_string(),
            f8: 400,
            f9: true
        }
    }));

    // Now serialize and deserialize it.
    let d2_se = serde_json::to_string(&d2)?;
    println!("Data Two V3 Se: {}", d2_se);

    let d2_de: EnumData = serde_json::from_str(&d2_se)?;

     println!("Data Two V3 De: {:?}", d2_de);

    assert_eq!(d2_de, EnumData::Two(EnumDataTwo {
        f1: "Another String".to_string(),
        f2: 100,
        f3: EnumDataTwoV1::V3 {
            f7: "Inner String V3".to_string(),
            f8: 400,
            f9: true
        }
    }));

    Ok(())
}
