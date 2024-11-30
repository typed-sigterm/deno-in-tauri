import ts from '@typed-sigterm/eslint-config';

export default ts({
  jsonc: true,
  toml: true,
  typescript: true,
  vue: true,
  yaml: true,
  ignores: ['**/target/**'],
});
