import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

import expressiveCode from "astro-expressive-code";

export default defineConfig({
  integrations: [
    expressiveCode(),
    starlight({
    title: "BELLE site",
    social: {
      github: "https://github.com/BlueGummi/belle",
    },
    sidebar: [
      {
        label: "Program Documentation",
        items: [
          { label: "Overview", slug: "" },
          { label: "Assembler", slug: "basm" },
          { label: "Emulator", slug: "belle" },
          { label: "Disassembler", slug: "bdump" },
          { label: "Utilities", slug: "btils" },
        ],
      },
      {
        label: "ISA and hardware",
        items: [
          { label: "Overview", slug: "overview" },
          { label: "Encoding", slug: "encoding" },
          { label: "Instructions", slug: "instructions" },
          { label: "CPU specification", slug: "cpu-core" },
          { label: "Memory", slug: "memory" },
        ],
      },
    ],
  })],
  devOptions: {
    proxy: {
      "/api": {
        target: "https://belle-demo.vercel.app",
        changeOrigin: true,
        pathRewrite: { "^/api": "/api" },
      },
    },
  },
});
