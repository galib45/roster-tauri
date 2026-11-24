<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import ModeSwitcher from "$lib/components/mode-switcher.svelte";

  let name = $state("");
  let greetMsg = $state("");
  let loading = $state(false);
  let error = $state("");

  async function greet(event: SubmitEvent) {
    event.preventDefault();

    if (!name.trim()) return;

    loading = true;
    error = "";
    greetMsg = "";

    try {
      greetMsg = await invoke<string>("greet", {
        name: name.trim(),
      });
    } catch (err) {
      error = String(err);
    } finally {
      loading = false;
    }
  }
</script>

<!-- <main class="flex bg-background p-6 justify-center"> -->
  <Card.Root class="w-fit h-fit">
    <Card.Header class="items-center text-center">
      <Card.Title class="text-2xl font-bold">
        Welcome to Tauri + Svelte
        <ModeSwitcher />
      </Card.Title>

      <Card.Description>
        Your desktop application is ready.
      </Card.Description>
    </Card.Header>

    <Card.Content class="space-y-6 flex items-center">
      <div class="flex items-center justify-center gap-6">
        <img src="/vite.svg" class="h-12 w-12" alt="Vite" />
        <img src="/tauri.svg" class="h-12 w-12" alt="Tauri" />
        <img src="/svelte.svg" class="h-12 w-12" alt="Svelte" />
      </div>

      <form onsubmit={greet} class="flex space-x-3">
        <Input id="greet-input" placeholder="Enter your name..." bind:value={name} />
        <Button type="submit" class="px-16" disabled={loading || !name.trim()} >
          {loading ? "Greeting..." : "Greet"}
        </Button>
      </form>

      {#if greetMsg}
        <div class="rounded-lg border bg-muted p-4 text-center text-sm">
          {greetMsg}
        </div>
      {:else if error}
        <div class="rounded-lg border border-destructive/50 bg-destructive/10 p-4 text-center text-sm text-destructive">
          {error}
        </div>
      {/if}
    </Card.Content>

    <Card.Footer class="justify-center">
      <p class="text-xs text-muted-foreground">
        Powered by Tauri, Svelte and shadcn-svelte
      </p>
    </Card.Footer>
  </Card.Root>
<!-- </main> -->
