/**
 * A generated module for Potato functions
 *
 * This module has been generated via dagger init and serves as a reference to
 * basic module structure as you get started with Dagger.
 *
 * Two functions have been pre-created. You can modify, delete, or add to them,
 * as needed. They demonstrate usage of arguments and return types using simple
 * echo and grep commands. The functions can be called from the dagger CLI or
 * from one of the SDKs.
 *
 * The first line in this comment block is a short description line and the
 * rest is a long description with more detail on the module's purpose or usage,
 * if appropriate. All modules should have a short description.
 */

import { dag, Container, Directory, object, func } from "@dagger.io/dagger"

@object()
// eslint-disable-next-line @typescript-eslint/no-unused-vars
class Potato {
  /**
   * Returns a container that echoes whatever string argument is provided
   */
  @func()
  containerEcho(stringArg: string): Container {
    // return dag.directory().from("alpine:latest").withExec(["echo", stringArg])
    // const modules = dag.container().from("alpine:latest")
    //     .directory(".");
    // return modules.withExec(["echo", stringArg])
    return dag.container().from("rust:latest").withExec(["echo", stringArg])
        // .withExec(["echo", stringArg])
        // .withExec(["ls", "-la"]);
  }

  @func()
  rustBuild(args: string[]): Directory {
    // return dag.container().from("rust:latest").withExec(["ls", "-la"])
    // return dag.container().from("rust:latest").withExec(["cargo build"])
    return dag
        .cargo()
        .build(args)
  }

  @func()
  containerBuild(): Container {
    return dag.container().from("rust:latest").withExec(["cargo build"])
  }

    /**
     * Returns a string that says "Hello Daggernauts!"
     */
  @func()
  helloWorld(): string {
    return "Hello world!"
  }

  /**
   * Returns lines that match a pattern in the files of the provided Directory
   */
  @func()
  async grepDir(directoryArg: Directory, pattern: string): Promise<string> {
    return dag
      .container()
      .from("alpine:latest")
      .withMountedDirectory("/mnt", directoryArg)
      .withWorkdir("/mnt")
      .withExec(["grep", "-R", pattern, "."])
      .stdout()
  }
}
