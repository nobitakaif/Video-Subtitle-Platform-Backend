import { prisma } from "@repo/db/client";
import { Elysia } from "elysia";
import { UserAuth } from "./module/auth";


const app = new Elysia({prefix : "/api/v1"})
  .use(UserAuth)
  .listen(8000);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);
