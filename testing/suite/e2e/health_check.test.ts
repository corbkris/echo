import { assertEquals } from "@std/assert";
import { config } from "../common.ts";

Deno.test("healthcheck_test", async () => {
  const res = await fetch(`${config.account_server}/health_check`);
  await res.body?.cancel();
  assertEquals(res.status, 200);
});
