<script lang="ts">
    import { goto } from "$app/navigation";
    import { Button } from "$lib/components/ui/button";
    import * as  Field from "$lib/components/ui/field";
    import * as InputGroup from "$lib/components/ui/input-group";
    import { PlusIcon, SearchIcon } from "@lucide/svelte";

    import { getAppState } from "$lib/state.svelte";
    import { createTable, FlexRender } from "@tanstack/svelte-table";
    import { features, columns } from "./columns";
    import * as Table from "$lib/components/ui/table";
    let appState = getAppState();
    const table = createTable({
      features, columns,
      get data() { return appState.organizations }
    });
</script>

<div class="w-full px-4 space-y-4">
<div id="page-header" class="w-full flex justify-between h-fit">
    <div>
        <div id="page-title" class="text-xl font-semibold">Organizations</div>
        <div id="page-description" class="text-xs my-1">Manage your organizations</div>
    </div>
    <div>
        <Button variant="outline" onclick={() => goto("/organizations/new")}>
            <PlusIcon class="size-4" /> Add organization
        </Button>
    </div>
</div>

<Field.Field class="w-full">
	<!-- <Field.Label for="inline-start-input">Input</Field.Label> -->
	<InputGroup.Root>
		<InputGroup.Input
		    id="inline-start-input" class="text-sm"
			placeholder="Search organizations..." />
		<InputGroup.Addon align="inline-start">
			<SearchIcon class="text-muted-foreground" />
		</InputGroup.Addon>
	</InputGroup.Root>
	<!-- <Field.Description>Icon positioned at the start.</Field.Description> -->
</Field.Field>

<div class="rounded-md border">
    <Table.Root>
        <Table.Header>
            {#each table.getHeaderGroups() as headerGroup}
                <Table.Row>
                    {#each headerGroup.headers as header}
                        <Table.Head>
                            {#if !header.isPlaceholder}
                                <FlexRender {header} />
                            {/if}
                        </Table.Head>
                    {/each}
                </Table.Row>
            {/each}
        </Table.Header>

        <Table.Body>
            {#each table.getRowModel().rows as row}
                <Table.Row>
                    {#each row.getAllCells() as cell}
                        <Table.Cell>
                            <FlexRender {cell} />
                        </Table.Cell>
                    {/each}
                </Table.Row>
            {:else}
                <Table.Row>
                    <Table.Cell colspan={columns.length}>
                        No organizations found.
                    </Table.Cell>
                </Table.Row>
            {/each}
        </Table.Body>
    </Table.Root>
</div>
</div>
