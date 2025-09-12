<script lang="ts">
  import * as Tabs from "$lib/components/ui/tabs/index.js";

  import GetSoftwareTab from "$lib/tabs/GetSoftwareTab.svelte";
  import WelcomeTab from "$lib/tabs/WelcomeTab.svelte";

  let tabs = [
    { name: "welcome", title: "Welcome", component: WelcomeTab },
    { name: "getsoftware", title: "Get Software", component: GetSoftwareTab },
  ];

  let activeTab = $state("welcome");

  function handleTabChange(tab: string) {
    activeTab = tab;
  }
</script>

<Tabs.Root value={activeTab} class="m-5 select-none">
  <Tabs.List class="self-center">
    {#each tabs as tab}
      <Tabs.Trigger onclick={() => (activeTab = tab.name)} value={tab.name}
        >{tab.title}</Tabs.Trigger
      >
    {/each}
  </Tabs.List>
  {#each tabs as tab}
    <Tabs.Content value={tab.name}>
      <tab.component tabChange={handleTabChange} />
    </Tabs.Content>
  {/each}
</Tabs.Root>
