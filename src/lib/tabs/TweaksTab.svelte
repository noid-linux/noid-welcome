<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  import { Rocket } from "@lucide/svelte";

  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import { Button } from "$lib/components/ui/button";

  interface Props {
    tabChange: (tab: string) => void;
  }

  let { tabChange }: Props = $props();

  let tweaks = $state([
    {
      name: "qemu-virt-manager",
      title: "Install QEMU & virt-manager",
      context:
        "This will execute a script to install qemu and virt-manager with all the dependencies needed for a smooth virtualization experience.",
      command: "install_qemu_vmm",
    },
    {
      name: "oxidize-system",
      title: "Oxidize Your System",
      context:
        "This will install a collection of Rust based utils and add a bunch of aliases to your shell config.",
      command: "oxidize_system",
    },
  ]);
</script>

<main class="flex flex-col gap-2 items-center">
  <div>
    <h1 class="text-4xl text-center">QOL Tweaks</h1>
    <p class="text-center">a few handy tweaks you might find useful</p>
  </div>

  <div class="flex flex-wrap gap-3 justify-center">
    {#each tweaks as tweak}
      <Tooltip.Provider>
        <Tooltip.Root delayDuration={250}>
          <Tooltip.Trigger>
            <button
              class="flex flex-col items-center hover:scale-[1.1] duration-500 ease-in-out"
              onclick={() => {
                invoke("run_tweak", { tweak: tweak.command });
              }}
            >
              <img class="h-32" src="tweaks/{tweak.name}.png" alt="" />
              <span>{tweak.title}</span>
            </button>
          </Tooltip.Trigger>
          <Tooltip.Content class="max-w-64">
            <span>{tweak.context}</span>
          </Tooltip.Content>
        </Tooltip.Root>
      </Tooltip.Provider>
    {/each}
  </div>
  <div class="flex mt-10">
    <Button
      onclick={() => {
        invoke("disable_startup");
        getCurrentWindow().close();
      }}
      size="lg"
      ><div class="flex items-center gap-1">
        <span>Finish Setup</span><Rocket />
      </div></Button
    >
  </div>
</main>
