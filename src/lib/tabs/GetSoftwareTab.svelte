<script lang="ts">
  import { open } from "@tauri-apps/plugin-shell";
  import { invoke } from "@tauri-apps/api/core";

  import { ArrowBigRight, Download } from "@lucide/svelte";

  import { ScrollArea } from "$lib/components/ui/scroll-area";
  import { Button } from "$lib/components/ui/button";

  interface Props {
    tabChange: (tab: string) => void;
  }

  let { tabChange }: Props = $props();

  let xbpsPackages = $state([
    { name: "spotify-client", selected: false },
    { name: "discord", selected: false },
    { name: "obsidian", selected: false },
    { name: "brave", selected: true },
    { name: "librewolf", selected: false },
    { name: "vscodium", selected: false },
    { name: "zen-browser", selected: false },
    { name: "ferdium", selected: false },
    { name: "draw-io", selected: false },
    { name: "onlyoffice", selected: false },
    { name: "cinny-desktop", selected: false },
    { name: "freetube", selected: false },
  ]);

  let selectedXbpsPackages = $derived(
    xbpsPackages.filter((pkg) => pkg.selected).map((pkg) => pkg.name),
  );
</script>

<main class="flex flex-col gap-2 items-center">
  <div>
    <h1 class="text-4xl text-center">Get Extra Packages</h1>
    <p class="text-center">
      Noid comes with a custom xbps repo enabled for extra packages
    </p>
  </div>
  <div class="flex flex-wrap gap-3 justify-center">
    {#each xbpsPackages as pkg}
      <button
        class="flex flex-col items-center gap-1 hover:gap-0 hover:grayscale-0 hover:scale-[1.1] duration-500 ease-in-out"
        class:grayscale-80={!pkg.selected}
        onclick={() => {
          pkg.selected = !pkg.selected;
        }}
      >
        <img class="h-16" src="xbps_packages/{pkg.name}.png" alt="" />
        <span>{pkg.name}</span>
      </button>
    {/each}
    <button
      class="flex flex-col items-center gap-1 grayscale-75 hover:gap-0 hover:grayscale-0 hover:scale-[1.1] duration-500 ease-in-out"
      onclick={() => open("https://git.ch-naseem.com/noid-linux/xbps-repo")}
    >
      <img class="h-16" src="icon.png" alt="" />
      <span>see more</span>
    </button>
  </div>

  <div class="flex justify-between items-center w-full">
    <Button
      variant="secondary"
      size="lg"
      onclick={() => {
        invoke("install_xbps_packages", {
          packages: selectedXbpsPackages,
        });
      }}><Download /> Install Selected</Button
    >
    <ScrollArea
      class="w-82 rounded-sm border bg-popover p-1.5"
      orientation="horizontal"
    >
      <span class="text-nowrap select-all"
        >ndpm install
        {#each xbpsPackages as pkg}
          {#if pkg.selected}
            {pkg.name}&nbsp;
          {/if}
        {/each}
      </span>
    </ScrollArea>
    <Button
      onclick={() => {
        tabChange("ndpm");
      }}
      size="lg"
      ><div class="flex items-center gap-1">
        <span>Next</span><ArrowBigRight />
      </div></Button
    >
  </div>
</main>

<style>
  img {
    pointer-events: none;
  }
</style>
