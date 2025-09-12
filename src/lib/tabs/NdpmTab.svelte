<script lang="ts">
  import { ArrowBigRight } from "@lucide/svelte";

  import { Button } from "$lib/components/ui/button";

  interface Props {
    tabChange: (tab: string) => void;
  }

  let { tabChange }: Props = $props();
</script>

<main class="space-y-1">
  <img class="rounded" src="xbps-to-ndpm.png" alt="" />
  <p>
    ndpm is a wrapper around XBPS that offers a more user-friendly package
    management experience, with a familiar syntax inspired by popular package
    managers such as apt and dnf. It also provides the capability to manage
    AppImages.
  </p>
  <div class="flex justify-end mt-1.75">
    <Button
      onclick={() => {
        tabChange("tweaks");
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
