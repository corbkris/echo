import { assertEquals, assertNotEquals } from "@std/assert";
import { config } from "../../common.ts";
import { faker } from "npm:@faker-js/faker";

Deno.test("basic_signup", async () => {
  const username = faker.internet.username();
  const req_body = {
    signup: {
      username: username,
      password: faker.internet.password(),
    },
  };
  const res = await fetch(`${config.account_server}/accounts/signup/basic`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(req_body),
  });

  assertEquals(res.status, 201);
  const res_body = await res.json();
  assertEquals(res_body.signup.username, username);
  assertNotEquals(res_body.signup.recovery_key, "");
});
