import { spawnSync } from "node:child_process";

export function resolveRepository() {
  const configured = process.env.RELEASE_REPOSITORY?.trim() || process.env.GITHUB_REPOSITORY?.trim();
  if (configured) return configured;

  const remote = spawnSync("git", ["config", "--get", "remote.origin.url"], {
    encoding: "utf8",
  });
  const repository = parseGitHubRepository(remote.stdout || "");
  if (repository) return repository;

  throw new Error(
    "Unable to determine the GitHub repository. Set RELEASE_REPOSITORY=owner/repository.",
  );
}

function parseGitHubRepository(remote) {
  const value = remote.trim();
  const match = value.match(/github\.com[/:]([^/]+\/[^/]+?)(?:\.git)?$/i);
  return match?.[1] || null;
}
