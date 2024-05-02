/**
 * NOTE:
 * This file is used to declare the module so that the types will be available when the `jsmodule:` path is used.
 * Other files such as `[moduleName].d.ts` are automatically created by the tsc command.
 */

// declare module 'jsmodule:[moduleName]' {
//   import moduleName = require('moduleName');
//   export = moduleName;
// }

declare module 'jsmodule:event' {
  import event = require('event');
  export = event;
}
