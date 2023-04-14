mod cframe;
mod instance;

use std::sync::Arc;

use mlua::{Lua, UserData, UserDataMethods};
use rbx_dom_weak::InstanceBuilder;

use crate::{
    remodel_context::RemodelContext,
    value::{
        Color3Value, Color3uint8Value, Vector2Value, Vector2int16Value, Vector3Value,
        Vector3int16Value,
    },
};

use cframe::CFrameUserData;
pub use instance::LuaInstance;

pub struct RobloxApi;

impl RobloxApi {
    pub fn inject(context: &Lua) -> mlua::Result<()> {
        context.globals().set("Instance", Instance)?;
        context.globals().set("Color3", Color3)?;
        context.globals().set("Color3uint8", Color3uint8)?;
        context.globals().set("Vector2", Vector2)?;
        context.globals().set("Vector2int16", Vector2int16)?;
        context.globals().set("Vector3", Vector3)?;
        context.globals().set("Vector3int16", Vector3int16)?;
        context.globals().set("CFrame", CFrameUserData)?;

        Ok(())
    }
}

struct Instance;

impl UserData for Instance {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("new", |context, class_name: String| {
            let database = rbx_reflection_database::get();

            if !database.classes.contains_key(class_name.as_str()) {
                return Err(mlua::Error::external(format!(
                    "'{}' is not a valid class of Instance.",
                    class_name,
                )));
            }

            let master_tree = RemodelContext::get(context)?.master_tree;
            let mut master_handle = master_tree.lock().unwrap();

            let builder = InstanceBuilder::new(class_name);

            let root_id = master_handle.root_ref();
            let id = master_handle.insert(root_id, builder);

            Ok(LuaInstance::new(Arc::clone(&master_tree), id))
        });
    }
}

struct Color3;

impl UserData for Color3 {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function("new", |_context, (r, g, b): (f32, f32, f32)| {
            Ok(Color3Value::new(rbx_dom_weak::types::Color3::new(r, g, b)))
        });
        methods.add_function("fromRGB", |_context, (r, g, b): (f32, f32, f32)| {
            Ok(Color3Value::new(rbx_dom_weak::types::Color3::new(
                r / 255.0,
                g / 255.0,
                b / 255.0,
            )))
        });
    }
}

struct Color3uint8;

impl UserData for Color3uint8 {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function("new", |_context, (r, g, b): (u8, u8, u8)| {
            Ok(Color3uint8Value::new(
                rbx_dom_weak::types::Color3uint8::new(r, g, b),
            ))
        });
    }
}

struct Vector2;

impl UserData for Vector2 {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function("new", |_context, (x, y): (Option<f32>, Option<f32>)| {
            Ok(Vector2Value::new(rbx_dom_weak::types::Vector2::new(
                x.unwrap_or(0.0),
                y.unwrap_or(0.0),
            )))
        });
    }
}

struct Vector2int16;

impl UserData for Vector2int16 {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function("new", |_context, (x, y): (Option<i16>, Option<i16>)| {
            Ok(Vector2int16Value::new(
                rbx_dom_weak::types::Vector2int16::new(x.unwrap_or(0), y.unwrap_or(0)),
            ))
        })
    }
}

struct Vector3;

impl UserData for Vector3 {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function(
            "new",
            |_context, (x, y, z): (Option<f32>, Option<f32>, Option<f32>)| {
                Ok(Vector3Value::new(rbx_dom_weak::types::Vector3::new(
                    x.unwrap_or(0.0),
                    y.unwrap_or(0.0),
                    z.unwrap_or(0.0),
                )))
            },
        );
    }
}

struct Vector3int16;

impl UserData for Vector3int16 {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function(
            "new",
            |_context, (x, y, z): (Option<i16>, Option<i16>, Option<i16>)| {
                Ok(Vector3int16Value::new(
                    rbx_dom_weak::types::Vector3int16::new(
                        x.unwrap_or(0),
                        y.unwrap_or(0),
                        z.unwrap_or(0),
                    ),
                ))
            },
        )
    }
}
