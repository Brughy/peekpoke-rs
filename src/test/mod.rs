use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    const ARR : [u64; 53] = [
        0x30303030,
 		0x30303030,
		0x30303030,
		0x30303030,
		0x30303030,
		0x30303030,
		0x30303030,
		0x30303030,
 		0x30303030,
 		0xa303030,
 		0x310a3030,
 		0x31310a30,
 		0x3131310a,
		0x31313131,
		0x31313131,
		0x31313131,
		0x31313131,
 		0x31313131,
 		0x31313131,
 		0x31313131,
 		0x31313131,
		0x31313131,
 		0xa313131,
		0x320a3131,
		0x32320a31,
		0x3232320a,
 		0x32323232,
 		0x32323232,
 		0x32323232,
		0x32323232,
 		0x32323232,
		0x32323232,
		0x32323232,
 		0x32323232,
 		0x32323232,
 		0xa323232,
 		0x330a3232,
 		0x33330a32,
 		0x3333330a,
 		0x33333333,
 		0x33333333,
 		0x33333333,
 		0x33333333,
 		0x33333333,
 		0x33333333,
 		0x33333333,
 		0x33333333,
 		0x33333333,
 		0xa333333,
 		0xa3333,
 		0xa33,
 		0xa,
 		0x0 ];

    #[test]
    fn read() {
    
        let f = open_mmap("src/test/a");
	
        let mut a = MyCfgAccess {
            offset: 0,
            size: 4,
            mode: "r".to_string(),
            data: 0,
            num: 1,
        };
	
        for i in 0..ARR.len() {
            a.offset = i as u64;
            let v = op(&f, &mut a);
            assert_eq!(v as U, ARR[i] as U);
        }
    }
    
    #[test]
    fn write_and_read() {
    
        let f = open_mmap("src/test/a");
	
        let mut a = MyCfgAccess {
            offset: 0,
            size: 4,
            mode: "r".to_string(),
            data: 0,
            num: 1,
        };
	
	//read
        for i in 0..ARR.len() {
            a.offset = i as u64;
            let v = op(&f, &mut a);
            assert_eq!(v as U, ARR[i] as U);
        }
		
	//write
	a.offset = 0;
	a.mode= "w".to_string();
	a.data = 0x35353535;
	op(&f, &mut a);
	
	//read
	a.mode = "r".to_string();
	a.offset = 0;
        let v = op(&f, &mut a);
        assert_eq!(v as U, 0x35353535 as U);
	a.offset = 1;
        let v = op(&f, &mut a);
        assert_eq!(v as U, 0x30353535 as U);
	a.offset = 2;
        let v = op(&f, &mut a);
        assert_eq!(v as U, 0x30303535 as U);	
	a.offset = 3;
        let v = op(&f, &mut a);
        assert_eq!(v as U, 0x30303035 as U);	
	
        for i in 4..ARR.len() {
            a.offset = i as u64;
            let v = op(&f, &mut a);
            assert_eq!(v as U, ARR[i] as U);
        }
    }
     
    #[test]
    fn write_and_read_restore() {
    
        let f = open_mmap("src/test/a");
	
        let mut a = MyCfgAccess {
            offset: 0,
            size: 4,
            mode: "r".to_string(),
            data: 0,
            num: 1,
        };
	
	//write
	a.offset = 0;
	a.mode= "w".to_string();
	a.data = 0x30303030;
	op(&f, &mut a);
	
	//read
	a.mode= "r".to_string();
        for i in 0..ARR.len() {
            a.offset = i as u64;
            let v = op(&f, &mut a);
            assert_eq!(v as U, ARR[i] as U);
        }
    }    

}
