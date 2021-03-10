struct NumWrapper(u8);
pub struct CNumWrapper(NumWrapper);

impl Drop for CNumWrapper {
    fn drop(&mut self) {
        println!("dropping CNumWrapper (val={})", self.0.0);
    }
}

#[no_mangle]
pub extern fn raw_numwrapper(num: u8) -> *mut CNumWrapper {
    let val = CNumWrapper(NumWrapper(num));
    let boxed = Box::new(val);
    let raw = Box::into_raw(boxed);

    raw
}

/* CNumWrapper is dropped */
#[no_mangle]
pub extern fn raw_to_num_box(cnw_raw: *mut CNumWrapper) -> u8 {
    let boxed = unsafe { Box::from_raw(cnw_raw) };
    let nw = *boxed;
    let num = nw.0.0;

    num
}

/* CNumWrapper is NOT dropped */
#[no_mangle]
pub extern fn raw_to_num_no_box(cnw_raw: *mut CNumWrapper) -> u8 {
    let cnw: &CNumWrapper = unsafe { &*cnw_raw };
    let num = cnw.0.0;

    num
}

/* CNumWrapper is NOT dropped */
#[no_mangle]
pub extern fn ref_to_num(cnw_ref: &mut CNumWrapper) -> u8 {
    let num = cnw_ref.0.0;

    num
}

// Returning a reference won't compile without lifetimes.
/*
#[no_mangle]
pub extern fn ref_numwrapper(u: u8) -> &CNumWrapper {
    unimplemented!()
}
*/
