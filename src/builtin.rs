use crate::{
    evaluator::{self},
    object,
};

pub fn builtins(name: &str) -> Option<object::Builtin> {
    match name {
        "len" => Some(object::Builtin { func: len }),
        "first" => Some(object::Builtin { func: first }),
        "last" => Some(object::Builtin { func: last }),
        "rest" => Some(object::Builtin { func: rest }),
        "push" => Some(object::Builtin { func: push }),
        "puts" => Some(object::Builtin { func: puts }),
        _ => None,
    }
}

fn len(args: &[Box<dyn object::Object>]) -> Box<dyn object::Object> {
    if args.len() != 1 {
        return evaluator::new_error(format!(
            "worng number of arguments. got={}, want=1",
            args.len()
        ));
    }

    if let Some(obj) = args[0].as_any().downcast_ref::<object::StringObject>() {
        Box::new(object::Integer {
            value: obj.value.len() as i64,
        })
    } else if let Some(obj) = args[0].as_any().downcast_ref::<object::Array>() {
        Box::new(object::Integer {
            value: obj.elements.len() as i64,
        })
    } else {
        evaluator::new_error(format!(
            "argument to \"len\" not supported, got {}",
            args[0].object_type(),
        ))
    }
}

fn first(args: &[Box<dyn object::Object>]) -> Box<dyn object::Object> {
    if args.len() != 1 {
        return evaluator::new_error(format!(
            "worng number of arguments. got={}, want=1",
            args.len()
        ));
    }

    if args[0].object_type() != object::ARRAY_OBJ {
        return evaluator::new_error(format!(
            "argument to \"first\" must be ARRAY, got {}",
            args[0].object_type(),
        ));
    }

    let array = args[0].as_any().downcast_ref::<object::Array>().unwrap();
    if !array.elements.is_empty() {
        return array.elements[0].clone_box();
    }

    Box::new(object::Null)
}

fn last(args: &[Box<dyn object::Object>]) -> Box<dyn object::Object> {
    if args.len() != 1 {
        return evaluator::new_error(format!(
            "worng number of arguments. got={}, want=1",
            args.len()
        ));
    }

    if args[0].object_type() != object::ARRAY_OBJ {
        return evaluator::new_error(format!(
            "argument to \"first\" must be ARRAY, got {}",
            args[0].object_type(),
        ));
    }

    let array = args[0].as_any().downcast_ref::<object::Array>().unwrap();
    let length = array.elements.len();
    if !array.elements.is_empty() {
        return array.elements[length - 1].clone_box();
    }

    Box::new(object::Null)
}

fn rest(args: &[Box<dyn object::Object>]) -> Box<dyn object::Object> {
    if args.len() != 1 {
        return evaluator::new_error(format!(
            "worng number of arguments. got={}, want=1",
            args.len()
        ));
    }

    if args[0].object_type() != object::ARRAY_OBJ {
        return evaluator::new_error(format!(
            "argument to \"first\" must be ARRAY, got {}",
            args[0].object_type(),
        ));
    }

    let array = args[0].as_any().downcast_ref::<object::Array>().unwrap();
    if !array.elements.is_empty() {
        return Box::new(object::Array {
            elements: array.elements[1..].iter().map(|e| e.clone_box()).collect(),
        });
    }

    Box::new(object::Null)
}

fn push(args: &[Box<dyn object::Object>]) -> Box<dyn object::Object> {
    if args.len() != 2 {
        return evaluator::new_error(format!(
            "worng number of arguments. got={}, want=2",
            args.len()
        ));
    }

    if args[0].object_type() != object::ARRAY_OBJ {
        return evaluator::new_error(format!(
            "argument to \"first\" must be ARRAY, got {}",
            args[0].object_type(),
        ));
    }

    let array = args[0].as_any().downcast_ref::<object::Array>().unwrap();
    let mut new_elements: Vec<Box<dyn object::Object>> =
        array.elements.iter().map(|e| e.clone_box()).collect();
    new_elements.push(args[1].clone_box());

    Box::new(object::Array {
        elements: new_elements,
    })
}

fn puts(args: &[Box<dyn object::Object>]) -> Box<dyn object::Object> {
    for arg in args {
        println!("{}", arg.inspect());
    }

    Box::new(object::Null)
}
