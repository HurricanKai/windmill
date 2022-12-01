import { Select } from "https://deno.land/x/cliffy@v0.25.4/prompt/select.ts";
import { GlobalOptions } from "./types.ts";
import { colors } from "https://deno.land/x/cliffy@v0.25.4/ansi/colors.ts";
import { getAvailablePort } from "https://deno.land/x/port@1.0.0/mod.ts";
import { UserService } from "https://deno.land/x/windmill@v1.50.0/mod.ts";
import { Secret } from "https://deno.land/x/cliffy@v0.25.4/prompt/secret.ts";
import { Workspace } from "./workspace.ts";

export async function loginInteractive(remote: string) {
  let token: string | undefined;
  if (
    await Select.prompt({
      message: "How do you wanna login",
      options: [
        {
          name: "Browser",
          value: "b",
        },
        {
          name: "Token",
          value: "t",
        },
      ],
    }) === "b"
  ) {
    token = await browserLogin(remote);
  } else {
    token = await Secret.prompt("Enter your token");
  }

  return token;
}

export async function tryGetLoginInfo(
  opts: GlobalOptions,
): Promise<string | undefined> {
  if (opts.token) {
    return opts.token;
  }

  if (opts.email && opts.password) {
    return await UserService.login({
      requestBody: {
        email: opts.email,
        password: opts.password,
      },
    });
  }

  return undefined;
}

export async function browserLogin(
  baseUrl: string,
): Promise<string | undefined> {
  const port = await getAvailablePort();
  if (port == undefined) {
    console.log(colors.red.underline("failed to aquire port"));
    return undefined;
  }

  const server = Deno.listen({ transport: "tcp", port });
  console.log(`Login by going to ${baseUrl}/user/cli?port=${port}`);
  const firstConnection = await server.accept();
  const httpFirstConnection = Deno.serveHttp(firstConnection);
  const firstRequest = (await httpFirstConnection.nextRequest())!;
  const params = new URL(firstRequest.request.url!).searchParams;
  const token = params.get("token");
  const _workspace = params.get("workspace");
  await firstRequest?.respondWith(
    Response.redirect(baseUrl + "/user/cli-success", 302),
  );

  setTimeout(() => {
    httpFirstConnection.close();
    server.close();
  }, 10);
  return token ?? undefined;
}
