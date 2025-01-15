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
          { label: "CPU specification", slug: "hardware" },
          { label: "Memory", slug: "memory" },
        ],
      },
      {
	label: "Tutorials",
	items: [
	  { label: "Introduction", slug: "tutorial/tutorial1" },
	  { label: "Hello, world!", slug: "tutorial/tutorial2" },
	  { label: "Moving Things Around", slug: "tutorial/tutorial3" },
	  { label: "Math and more", slug: "tutorial/tutorial4" },
	  { label: "Branches and conditionals", slug: "tutorial/tutorial5" },
	  { label: "The Stack", slug: "tutorial/tutorial6" },
	  { label: "Addressing Modes", slug: "tutorial/tutorial7" },
	  { label: "The CPU itself", slug: "tutorial/tutorial8" },
	  { label: "A Look at the Binary", slug: "tutorial/tutorial9" },
	  { label: "Reverse engineering", slug: "tutorial/tutorial10" },
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
