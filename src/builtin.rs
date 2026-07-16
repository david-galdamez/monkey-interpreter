use crate::{
    evaluator::{self, new_error},
    object,
};

fn len(args: &[Box<dyn object::Object>]) -> Box<dyn object::Object> {
    if args.len() != 1 {
        return evaluator::new_error(format!(
            "worng number of arguments. got={}, want=1",
            args.len()
        ));
    }

    if let Some(obj) = args[0].as_any().downcast_ref::<object::StringObject>() {
        return Box::new(object::Integer {
            value: obj.value.len() as i64,
        });
    } else {
        return new_error(format!(
            "argument to \"len\" not supported, got {}",
            args[0].object_type(),
        ));
    }
}

pub fn builtins(name: &str) -> Option<object::Builtin> {
    match name {
        "len" => Some(object::Builtin { func: len }),
        _ => None,
    }
}
