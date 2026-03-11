import esbuild from "esbuild";

const watch = process.argv.includes("--watch");

const ctx = await esbuild.context({
  entryPoints: ["src/main.ts"],
  outfile: "dist/main.js",
  bundle: true,
  format: "cjs",
  target: ["es2018"],
  sourcemap: true,
  minify: false,
  platform: "browser"
});

if (watch) {
  await ctx.watch();
  console.log("[mist-breakout] watching...");
} else {
  await ctx.rebuild();
  await ctx.dispose();
  console.log("[mist-breakout] build done");
}
