use crate::{
    framework,
    wrapper::client::{
        self,
        raw::{
            TEEC_CloseSession, TEEC_Context, TEEC_FinalizeContext, TEEC_InitializeContext,
            TEEC_InvokeCommand, TEEC_OpenSession, TEEC_Operation, TEEC_Session, TEEC_LOGIN_PUBLIC,
            TEEC_SUCCESS,
        },
        Operation, ParamNone, ParamTmpRef,
    },
    StackVec,
};
use core::ptr::null_mut;

#[derive(Debug)]
pub struct Client {
    ctx: TEEC_Context,
    sess: TEEC_Session,
    return_origin: u32,
}

impl Client {
    pub fn new(mut ctx: TEEC_Context, name: &str, sess: TEEC_Session) -> framework::Result<Self> {
        Self::initialize_context(name, &mut ctx)?;
        Ok(Self {
            ctx,
            return_origin: u32::default(),
            sess,
        })
    }

    pub fn open_session(
        &mut self,
        destination: client::Uuid,
        operation: &mut TEEC_Operation,
    ) -> framework::Result<()> {
        let rslt = unsafe {
            TEEC_OpenSession(
                &mut self.ctx,
                &mut self.sess,
                destination.as_ptr(),
                TEEC_LOGIN_PUBLIC,
                null_mut(),
                operation,
                &mut self.return_origin,
            )
        };
        if rslt == TEEC_SUCCESS {
            Ok(())
        } else {
            Err(framework::Error::ClientCodeWithOrigin(
                rslt,
                self.return_origin,
            ))
        }
    }

    pub fn invoke_command<T, U>(&mut self, instance: T) -> framework::Result<U>
    where
        T: serde::Serialize,
        U: for<'a> serde::Deserialize<'a>,
    {
        let mut input_buffer = StackVec::new([0; 256], 0);
        let mut output_buffer = StackVec::new([0; 256], 0);

        framework::serialize(&instance, &mut input_buffer);
        let p0 = ParamTmpRef::new_input(&input_buffer);
        let p1 = ParamTmpRef::new_output(&mut output_buffer);
        let mut operation = Operation::new(p0, p1, ParamNone, ParamNone);
        let rslt = unsafe {
            TEEC_InvokeCommand(
                &mut self.sess,
                0,
                operation.as_mut_raw_ptr(),
                &mut self.return_origin,
            )
        };
        if rslt == TEEC_SUCCESS {
            let mut scratch = [0; 256];
            Ok(framework::deserialize(&output_buffer, &mut scratch))
        } else {
            Err(framework::Error::ClientCodeWithOrigin(
                rslt,
                self.return_origin,
            ))
        }
    }

    fn initialize_context(name: &str, ctx: &mut TEEC_Context) -> framework::Result<()> {
        let rslt = unsafe { TEEC_InitializeContext(name.as_ptr() as *const _, ctx as *mut _) };
        if rslt == TEEC_SUCCESS {
            Ok(())
        } else {
            Err(framework::Error::ClientCode(rslt))
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            TEEC_CloseSession(&mut self.sess as *mut _);
            TEEC_FinalizeContext(&mut self.ctx as *mut _);
        }
    }
}
