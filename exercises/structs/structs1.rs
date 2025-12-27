
struct ColorClassicStruct {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // 实例化经典C风格结构体，设置red=0, green=255, blue=0
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // 实例化元组结构体，元素依次为0, 255, 0
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_like_structs() {
        // 实例化单元结构体（无字段）
        let unit_like_struct = UnitLikeStruct;

        let message = format!("{:?}s are fun!", unit_like_struct);
        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
