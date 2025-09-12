<script lang="ts">
  import { open } from "@tauri-apps/plugin-shell";

  import { ArrowBigRight, Code, Mail } from "@lucide/svelte";

  import * as Tooltip from "$lib/components/ui/tooltip/index.js";

  import { Button } from "$lib/components/ui/button";

  interface Props {
    tabChange: (tab: string) => void;
  }

  let { tabChange }: Props = $props();
</script>

<main class="flex flex-col items-center gap-4">
  <h1 class="text-4xl">Welcome to Noid Linux</h1>
  <div class="flex items-center gap-8 mx-4">
    <img class="max-h-48" src="icon.png" alt="banner" />
    <p class="text-lg w-full">
      This guide will walk you through essential setup steps and introduce you
      to our custom tools. Ready to get started? <br /> <br /> Have questions or
      feedback? Don't hesitate to reach out.
    </p>
  </div>
  <div class="flex mt-3 justify-between w-full">
    <div class="flex gap-1">
      <Tooltip.Provider>
        <Tooltip.Root delayDuration={100}>
          <Tooltip.Trigger>
            <Button
              size="lg"
              variant="secondary"
              onclick={() => open("mailto:contact@ch-naseem.com")}
              ><div class="flex items-center gap-1">
                <Mail /><span>Contact</span>
              </div></Button
            >
          </Tooltip.Trigger>
          <Tooltip.Content>
            <span>contact@ch-naseem.com</span>
          </Tooltip.Content>
        </Tooltip.Root>
      </Tooltip.Provider>
      <Button
        size="lg"
        variant="secondary"
        onclick={() => open("https://git.ch-naseem.com/noid-linux")}
        ><div class="flex items-center gap-1">
          <Code /><span>Code</span>
        </div></Button
      >
    </div>
    <Button
      onclick={() => {
        tabChange("welcome");
      }}
      size="lg"
      ><div class="flex items-center gap-1">
        <span>Let's go!</span><ArrowBigRight />
      </div></Button
    >
  </div>
</main>

<style>
  img {
    pointer-events: none;
  }
</style>
