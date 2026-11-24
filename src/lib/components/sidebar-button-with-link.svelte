<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar"
    import { page } from "$app/state";

    let {item} = $props();

    function isActive(url: string) {
      if (url === "/") return page.url.pathname === "/";

      return page.url.pathname === url ||
        page.url.pathname.startsWith(url + "/");
    }
</script>

<Sidebar.MenuButton class="select-none"
  isActive={isActive(item.url)}
  tooltipContent={item.title}
>
{#snippet child({ props })}
<a href={item.url} {...props}>
  <item.icon class="size-4" />
  <span>{item.title}</span>
  </a>
  {/snippet}
</Sidebar.MenuButton>
