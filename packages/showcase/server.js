
const path = require("path");
const fs = require("fs");

const root = path.resolve(__dirname, "..", "..");
const public = path.resolve(__dirname, "pkg")
if(!fs.existsSync(public)) {
    console.error(`${public}ディレクトリがありません。`);
    process.exit()
}
/**
 * Symlink作成
 */
const assets_link = path.resolve(public, "assets");
fs.existsSync(assets_link) || fs.symlinkSync(path.resolve(root, "public", "assets"),assets_link,)

/**
 * index.html作成
 */
const ejs = require('ejs');
const res = ejs.renderFile(
    path.resolve(root, "index.ejs"),
    { scripts: 
      `
        <script>import("/showcase.js").then(init => init.default())</script>
        <style>
          .cafeteria-root li{
            margin-left: 1rem;
          }
          .cafeteria-root strong {
            font-weight: bold;
          }
          .cafeteria-root h2 {
            font-weight: bold;
            height: 2rem;
          }
        </style>
      ` },
    { escape: str => str }
)
res.then(content => fs.writeFileSync(path.resolve(__dirname, "pkg", "index.html"), content));

/**
 * サーバー実行
 */
const handler = require('serve-handler');
const http = require('http');

const server = http.createServer((request, response) => {
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
