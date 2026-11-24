
<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { Check, ChevronDown, Plus } from "@lucide/svelte";
  import { getProjectContext } from "$lib/state.svelte";

  const appState = getProjectContext();
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger>
    {#snippet child({ props })}
      <Sidebar.MenuButton {...props} class="border border-sidebar-border">
        <span class="flex-1 truncate text-left">
          {appState.selectedOrganization?.name ?? "Select organization"}
        </span>
        <ChevronDown class="size-4" />
      </Sidebar.MenuButton>
    {/snippet}
  </DropdownMenu.Trigger>

  <DropdownMenu.Content class="w-60" align="start">
    <DropdownMenu.Group>
      <DropdownMenu.Label>Switch organization</DropdownMenu.Label>

      {#each appState.organizations as organization (organization.id)}
        <DropdownMenu.Item
          onclick={() => appState.selectOrganization(organization.id)}
        >
          <span class="flex-1">{organization.name}</span>

          {#if appState.selectedOrganizationId === organization.id}
            <Check class="size-4" />
          {/if}
        </DropdownMenu.Item>
      {/each}
    </DropdownMenu.Group>

    <DropdownMenu.Separator />

    <DropdownMenu.Item>
      <Plus class="size-4" />
      Add organization
    </DropdownMenu.Item>
  </DropdownMenu.Content>
</DropdownMenu.Root>
