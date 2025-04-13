import { assertEquals, assertNotEquals } from "@std/assert";
import { config } from "../../common.ts";
import { faker } from "npm:@faker-js/faker";
import mailhog from "npm:mailhog";

const client = mailhog({
  host: "localhost",
  port: 8025,
});

Deno.test("login", async (t) => {
  const state = {
    basic_signup: {
      username: faker.internet.username(),
      password: faker.internet.password(),
    },
    managed_signup: {
      email: faker.internet.email(),
      username: faker.internet.username(),
      password: faker.internet.password(),
      code: "",
      req_id: "",
    },
  };
  await t.step("basic_signup", async () => {
    const req_body = {
      signup: {
        username: state.basic_signup.username,
        password: state.basic_signup.password,
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
    assertEquals(res_body.signup.username, state.basic_signup.username);
    assertNotEquals(res_body.signup.recovery_key, "");
  });

  await t.step("login with basic_signup info", async () => {
    const req_body = {
      login: {
        username: state.basic_signup.username,
        password: state.basic_signup.password,
      },
    };

    const res = await fetch(`${config.account_server}/accounts/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(req_body),
    });

    assertEquals(res.status, 200);
    assertNotEquals(res.headers.get("x-auth-token") ?? "", "");
    await res.text();
  });

  await t.step("send managed_signup request", async () => {
    const req_body = {
      signup: {
        email: state.managed_signup.email,
        username: state.managed_signup.username,
        password: state.managed_signup.password,
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
    state.managed_signup.req_id = res.headers.get("x-signup-req-id") ?? "";
    await res.text();
  });

  await t.step("search email for code", async () => {
    const message = await client.latestTo(state.managed_signup.email);
    state.managed_signup.code = message?.text ?? "";
  });

  await t.step("confirm managed_signup request", async () => {
    const res = await fetch(
      `${config.account_server}/accounts/signup/managed/${state.managed_signup.code}`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-signup-req-id": state.managed_signup.req_id,
        },
      },
    );

    assertEquals(res.status, 201);
    await res.text();
  });

  await t.step("login with managed_signup email", async () => {
    const req_body = {
      login: {
        email: state.managed_signup.email,
        password: state.managed_signup.password,
      },
    };

    const res = await fetch(`${config.account_server}/accounts/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(req_body),
    });

    assertEquals(res.status, 200);
    assertNotEquals(res.headers.get("x-auth-token") ?? "", "");
    await res.text();
  });

  await t.step("login with managed_signup username", async () => {
    const req_body = {
      login: {
        username: state.managed_signup.username,
        password: state.managed_signup.password,
      },
    };

    const res = await fetch(`${config.account_server}/accounts/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(req_body),
    });

    assertEquals(res.status, 200);
    assertNotEquals(res.headers.get("x-auth-token") ?? "", "");
    await res.text();
  });
});
