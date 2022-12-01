// deno-lint-ignore-file no-explicit-any
import { Command } from "https://deno.land/x/cliffy@v0.25.4/command/mod.ts";
import { Select } from "https://deno.land/x/cliffy@v0.25.4/prompt/select.ts";
import { GlobalOptions } from "./types.ts";
import { colors } from "https://deno.land/x/cliffy@v0.25.4/ansi/colors.ts";
import { getStore } from "./store.ts";
import { getAvailablePort } from "https://deno.land/x/port@1.0.0/mod.ts";
import { UserService } from "https://deno.land/x/windmill@v1.50.0/mod.ts";
import { resolveWorkspace } from "./context.ts";
import { Secret } from "https://deno.land/x/cliffy@v0.25.4/prompt/secret.ts";
import { setWorkspaceToken } from "./workspace.ts";
type Options = GlobalOptions;

async function login(opts: Options) {
  const workspace = await resolveWorkspace(opts);

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
    token = await browserLogin(workspace.remote);
  } else {
    token = await Secret.prompt("Enter your token");
  }

  setWorkspaceToken(workspace, token);
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
export async function getToken(baseUrl: string): Promise<string> {
  const baseStore = await getStore(baseUrl);
  try {
    return await Deno.readTextFile(baseStore + "token");
  } catch {
    console.log(
      colors.bold.underline.red(
        "You need to be logged in to do this! Run 'windmill login' to login.",
      ),
    );
    return Deno.exit(-1);
  }
}

const command = new Command()
  .description(
    "Log into windmill. The credentials are not stored, but the token they are exchanged for will be.",
  )
  .action(login as any);

export default command;
