use puniyu_plugin::prelude::*;

#[command(name = "echo", desc = "输出一段文本")]
#[arg(name = "msg", desc = "要输出的文本")]
async fn echo(ctx: &MessageContext) -> HandlerResult<HandlerAction> {
    let msg = ctx.arg("msg").and_then(|m| m.as_str()).unwrap();
    ctx.reply(msg.into()).await?;
    Ok(().into())
}
