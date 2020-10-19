import * as oak from "https://deno.land/x/oak/mod.ts";

import * as vars from "./vars.ts";

const app = new oak.Application();

// Register the logger
app.use(async (ctx: oak.Context, next: () => Promise<void>) => {
  await next();
  console.log(
    `${
      ctx.request.secure ? "Secure" : "Insecure"
    } ${ctx.request.method} from ${ctx.request.ip} for ${ctx.request.url}`,
  );
});

await app.listen(`0.0.0.0:${vars.PORT}`);
