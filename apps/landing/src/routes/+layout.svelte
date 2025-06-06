<script lang="ts">
	import "../app.css"
	import Header from "$lib/components/Header.svelte"

	let { children } = $props()
</script>

<div class="container mx-auto max-w-2xl px-6 pt-16">
	<Header />
	{@render children()}
</div>
