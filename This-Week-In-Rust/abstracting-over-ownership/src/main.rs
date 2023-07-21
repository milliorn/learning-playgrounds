#[derive(Debug)]
enum Value {
  String(String),
  Number(i64),
}

trait TryConvertValue<'a> {
  type Output;
  fn try_convert_value(value: &'a Value) -> Option<Self::Output>;
}

impl<'a> TryConvertValue<'a> for String {
  type Output = String;
  fn try_convert_value(value: &'a Value) -> Option<String> {
    match value {
      Value::String(string) => Some(string.clone()),
      Value::Number(number) => Some(number.to_string()),
    }
  }
}

impl<'a> TryConvertValue<'a> for &str {
  type Output = &'a str;
  fn try_convert_value(value: &'a Value) -> Option<&'a str> {
    match value {
      Value::String(string) => Some(string),
      Value::Number(_) => None,
    }
  }
}

impl<'a> TryConvertValue<'a> for i64 {
  type Output = i64;
  fn try_convert_value(value: &'a Value) -> Option<i64> {
    match value {
      Value::String(_) => None,
      Value::Number(number) => Some(*number),
    }
  }
}

trait CallbackArgs<'a> {
  type Output;
  fn convert(values: &'a [Value]) -> Option<Self::Output>;
}

impl<'a, A> CallbackArgs<'a> for (A,)
where
  A: TryConvertValue<'a>,
{
  type Output = (A::Output,);

  fn convert(values: &'a [Value]) -> Option<Self::Output> {
    Some((A::try_convert_value(&values[0])?,))
  }
}

impl<'a, A, B> CallbackArgs<'a> for (A, B)
where
  A: TryConvertValue<'a>,
  B: TryConvertValue<'a>,
{
  type Output = (A::Output, B::Output);

  fn convert(values: &'a [Value]) -> Option<Self::Output> {
    Some((
      A::try_convert_value(&values[0])?,
      B::try_convert_value(&values[1])?,
    ))
  }
}

trait Callback<Args>: 'static {
  fn invoke(&self, args: Args) -> Value;
}

impl<Func, A> Callback<(A,)> for Func
where
  Func: Fn(A) -> Value + 'static,
{
  fn invoke(&self, (a,): (A,)) -> Value {
    self(a)
  }
}

impl<Func, A, B> Callback<(A, B)> for Func
where
  Func: Fn(A, B) -> Value + 'static,
{
  fn invoke(&self, (a, b): (A, B)) -> Value {
    self(a, b)
  }
}

struct BoxedCallback(Box<dyn Fn(&[Value]) -> Value>);

impl BoxedCallback {
  pub fn new<Func, Args>(f: Func) -> Self
  where
    Args: for<'a> CallbackArgs<'a>,
    Func: Callback<Args> + for<'a> Callback<<Args as CallbackArgs<'a>>::Output>,
  {
    BoxedCallback(Box::new(move |args| {
      let args = Args::convert(args).unwrap();
      f.invoke(args)
    }))
  }

  pub fn invoke(&self, args: &[Value]) -> Value {
    (self.0)(args)
  }
}

fn main() {
  let square = BoxedCallback::new(|n: i64| Value::Number(n * n));
  dbg!(square.invoke(&[Value::Number(3)]));

  let to_upper = BoxedCallback::new(|s: &str| Value::String(s.to_uppercase()));
  let args = vec![Value::String("abc".to_owned())];
  dbg!(to_upper.invoke(&args));

  let append = BoxedCallback::new(|s: &str, n: i64| Value::String(format!("{}{}", s, n)));
  let args = vec![Value::String("abc".to_owned()), Value::Number(42)];
  dbg!(append.invoke(&args));
}
