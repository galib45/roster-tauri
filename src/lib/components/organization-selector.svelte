
<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { Check, ChevronDown, Plus, BuildingComplex } from "@lucide/svelte";

  let selectedOrganization = $state("City Hospital");

  const organizations = [
    "City Hospital",
    "Central Hospital",
    "Community Clinic",
  ];
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger>
    {#snippet child({ props })}
      <Sidebar.MenuButton
        {...props}
        class="border border-sidebar-border"
      >
        <!-- <div class="flex size-7 items-center justify-center rounded-md bg-muted">
          <BuildingComplex class="size-4" />
        </div> -->

        <span class="flex-1 truncate text-left">
          {selectedOrganization}
        </span>

        <ChevronDown class="size-4" />
      </Sidebar.MenuButton>
    {/snippet}
  </DropdownMenu.Trigger>

  <DropdownMenu.Content
    class="w-60"
    align="start"
    side="bottom"
  >
    <DropdownMenu.Group>
      <DropdownMenu.Label>
        Switch organization
      </DropdownMenu.Label>

      {#each organizations as organization}
        <DropdownMenu.Item
          onclick={() => (selectedOrganization = organization)}
        >
          <span class="flex-1">{organization}</span>

          {#if selectedOrganization === organization}
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
