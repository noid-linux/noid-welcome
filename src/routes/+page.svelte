<script lang="ts">
  import TabOne from "$lib/tabs/TabOne.svelte";
  import TabTwo from "$lib/tabs/TabTwo.svelte";

  import * as Tabs from "$lib/components/ui/tabs/index.js";

  let tabs = [
    { name: "tab1", title: "Tab One", component: TabOne },
    { name: "tab2", title: "Tab Two", component: TabTwo },
  ];
</script>

<Tabs.Root value="tab1" class="">
  <Tabs.List class="self-center">
    {#each tabs as tab}
      <Tabs.Trigger value={tab.name}>{tab.title}</Tabs.Trigger>
    {/each}
  </Tabs.List>
  {#each tabs as tab}
    <Tabs.Content value={tab.name}>
      <svelte:component this={tab.component} />
    </Tabs.Content>
  {/each}
</Tabs.Root>
