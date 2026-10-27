# Olostep MCP Server for Zed

**Live web search, scraping and crawling API for AI. Get structured, LLM-ready data from any site. Scale to 1M requests in a single batch at low cost.**

[Olostep](https://www.olostep.com) is web data infrastructure for AI. It provides live web search, scraping and crawling API to extract and structure data. This Zed extension adds the [Olostep MCP server](https://github.com/olostep/olostep-mcp-server) to Zed's Agent Panel: scraping (with JavaScript rendering), web search, answers with citations, site mapping, batch scraping and crawling.

The extension installs the [`olostep-mcp`](https://www.npmjs.com/package/olostep-mcp) package from npm and runs it with Zed's bundled Node.js. Nothing else needs to be installed.

## Setup

1. Install **Olostep MCP Server** from Zed's extensions page (`zed: extensions`).
2. Get an API key at [olostep.com/auth](https://olostep.com/auth).
3. Add the key to your Zed `settings.json`:

```json
{
  "context_servers": {
    "mcp-server-olostep": {
      "settings": {
        "olostep_api_key": "YOUR_OLOSTEP_API_KEY"
      }
    }
  }
}
```

Zed also shows this configuration when you enable the server in **Agent Panel → Settings → MCP Servers**.

## Tools

| Tool | What it does |
|---|---|
| `scrape_website` | Scrape a URL as markdown, HTML, JSON or text |
| `get_webpage_content` | Get a page's content as markdown |
| `search_web` | Structured web search results |
| `answers` | AI answers with source citations, optionally as JSON |
| `get_website_urls` | Find URLs on a site that match a query |
| `create_map` | List all URLs on a website |
| `batch_scrape_urls` | Scrape many URLs in one job |
| `get_batch_results` | Fetch the results of a batch |
| `create_crawl` | Crawl a website from a start URL |
| `get_crawl_results` | Fetch the results of a crawl |

## Example prompts

- "Use Olostep to read https://docs.python.org/3/whatsnew/3.13.html and summarize the breaking changes."
- "Search the web for the latest Next.js release and list the new features with sources."

## About Olostep

[Olostep](https://www.olostep.com) is web data infrastructure for AI. It provides live web search, scraping and crawling API to extract and structure data, used in production by AI companies to give their agents fresh, LLM-ready web data at scale.

- Website: https://www.olostep.com
- Documentation: https://docs.olostep.com

## License

MIT
