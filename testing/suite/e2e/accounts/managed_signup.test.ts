import mailhog from "npm:mailhog";
import { faker } from "npm:@faker-js/faker";
import { assertEquals } from "@std/assert";
import { config } from "../../common.ts";

const client = mailhog({
  host: "localhost",
  port: 8025,
});

Deno.test("managed_signup", async (t) => {
  const state = {
    req_id: "",
    code: "",
    email: "",
  };
  await t.step("send managed_signup request", async () => {
    const email = faker.internet.email();
    state.email = email;
    const req_body = {
      signup: {
        email: email,
        username: faker.internet.username(),
        password: faker.internet.password(),
      },
    };
    const res = await fetch(
      `${config.account_server}/accounts/signup/managed`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(req_body),
      },
    );

    assertEquals(res.status, 200);
    state.req_id = res.headers.get("x-signup-req-id") ?? "";
    await res.text();
  });

  await t.step("search email for code", async () => {
    const message = await client.latestTo(state.email);
    state.code = message?.text ?? "";
    console.log(message?.text);
  });

  await t.step("confirm managed_signup request", async () => {
    const res = await fetch(
      `${config.account_server}/accounts/signup/managed/${state.code}`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-signup-req-id": state.req_id,
        },
      },
    );

    assertEquals(res.status, 201);
    await res.text();
  });
});
