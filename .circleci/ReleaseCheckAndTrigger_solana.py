import os
import requests
import semver
from enum import Enum
from datetime import datetime, timedelta, UTC
from typing import Any

# Main release URL
RELEASE_URL = "https://pypi.org/pypi/certora-cli/json"


# Retrieve CircleCI API token from environment variables
CIRCLECI_TOKEN = os.getenv("CIRCLECI_API_TOKEN")
if not CIRCLECI_TOKEN:
    raise EnvironmentError("CIRCLECI_API_TOKEN environment variable is not set.")

# ci url for the SolanaExamples project
CIRCLECI_URL = "https://circleci.com/api/v2/project/gh/Certora/SolanaExamples/pipeline"


def fetch_package_info(url: str) -> dict[str, Any] | None:
    """
    Fetches package information from the given URL.

    Args:
        url (str): The URL to fetch the package information from.

    Returns:
        Optional[Dict[str, Any]]: The package information as a dictionary if successful, None otherwise.
    """
    try:
        response = requests.get(url)
        response.raise_for_status()
        return response.json()
    except requests.RequestException as e:
        print(f"Failed to fetch package info from {url}: {e}")
        return None

def get_latest_version(versions: list[str]) -> str:
    valid_versions = []
    for version in versions:
        try:
            # Try to parse the version, if it's valid, add to the list
            semver.VersionInfo.parse(version)
            valid_versions.append(version)
        except ValueError:
            # Skip invalid versions
            print(f"Skipping invalid version: {version}")
    
    # Sort the valid versions using semver library
    sorted_versions = sorted(valid_versions, key=semver.VersionInfo.parse)
    # The latest version is the last one in the sorted list
    return sorted_versions[-1]


def get_latest_release_date(info: dict[str, Any]) -> datetime:
    """
    Extracts the latest release date from the package information.

    Args:
        info (Dict[str, Any]): The package information.

    Returns:
        datetime: The latest release date.
    """
    last_release = get_latest_version(list(info["releases"]))
    upload_time = max(
        d["upload_time"] for d in info["releases"][last_release] if "upload_time" in d
    )
    return datetime.strptime(upload_time, '%Y-%m-%dT%H:%M:%S').replace(tzinfo=UTC)


# Trigger the CircleCI workflow if a new release has been made in the last 24 hours
def trigger_workflow_if_new():
    requests.post(
        CIRCLECI_URL,
        headers={ "Circle-Token": os.environ["CIRCLECI_API_TOKEN"], "Content-Type": "application/json" },
        json={
            "branch": "main",
        }
    ).raise_for_status()


def main():
    print("Checking for new releases of certora-cli...")
    info = fetch_package_info(RELEASE_URL)
    if not info:
        print("Failed to fetch package information.")
        exit(1)

    latest_release_date = get_latest_release_date(info)
    print(f"Last release date for certora-cli: {latest_release_date}")

    today = datetime.now(UTC)
    if today - timedelta(days=1) <= latest_release_date <= today:
        print("A new release has been made in the last 24 hours. Triggering workflow...")
        trigger_workflow_if_new()
    else:
        print("No new release in the last 24 hours. No action taken.")


if __name__ == "__main__":
    main()