const path = require("path");
const fs = require("fs/promises");

const root = path.resolve(__dirname, "..", "..");
const public = path.resolve(__dirname, "pkg")

const prepareShowcase = async () => {
  const statPublic = await fs.lstat(public);
  if(!statPublic.isDirectory()) {
      throw new Error(`${public}ディレクトリがありません。`)
  }

  /**
   * Symlink作成
   */
  const assets_link = path.resolve(public, "assets");
  const statAssets = await fs.lstat(assets_link).catch(() => false);
  if(!statAssets) {
    await fs.symlink(path.resolve(root, "public", "assets"),assets_link);
  }
  
  /**
   * index.html作成
   */
  const ejs = require('ejs');
  const content = await ejs.renderFile(
      path.resolve(root, "index.ejs"),
      { scripts: 
        `
          <script>import("/showcase.js").then(init => init.default())</script>
          <style>
            .cafeteria-nav li{
              margin-left: 1rem;
            }
            .cafeteria-nav strong {
              font-weight: bold;
            }
            .cafeteria-nav h2 {
              font-weight: bold;
              height: 2rem;
            }
          </style>
        ` },
      { escape: str => str }
  )
  await fs.writeFile(path.resolve(__dirname, "pkg", "index.html"),content)
}
/**
 * サーバー実行
 */
const handler = require('serve-handler');
const http = require('http');

const server = http.createServer(async (request, response) => {
  await prepareShowcase().catch(e => console.error(e));
  return handler(request, response,{
      public: path.resolve(__dirname, "pkg"),
      symlinks: true,
      cleanUrls: true,
      rewrites: [ { source: "**", destination : "/index.html" }]
  });
})
const PORT = 3001;
server.listen(PORT, () => {
  console.log(`Running at http://localhost:${PORT}`);
});
