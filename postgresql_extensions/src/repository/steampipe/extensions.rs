use std::sync::LazyLock;

static EXTENSIONS: LazyLock<Vec<SteampipeExtension>> = LazyLock::new(init_extensions);

#[expect(clippy::too_many_lines)]
fn init_extensions() -> Vec<SteampipeExtension> {
    vec![
        SteampipeExtension::new(
            "abuseipdb",
            "Steampipe plugin to query IP address abuse data and more from AbuseIPDB.",
            "https://github.com/turbot/steampipe-plugin-abuseipdb",
        ),
        SteampipeExtension::new(
            "airtable",
            "Steampipe plugin for querying Airtable.",
            "https://github.com/francois2metz/steampipe-plugin-airtable",
        ),
        SteampipeExtension::new(
            "aiven",
            "Steampipe plugin to query accounts, projects, teams, users and more from Aiven.",
            "https://github.com/turbot/steampipe-plugin-aiven",
        ),
        SteampipeExtension::new(
            "algolia",
            "Steampipe plugin for querying Algolia indexes, logs and more.",
            "https://github.com/turbot/steampipe-plugin-algolia",
        ),
        SteampipeExtension::new(
            "alicloud",
            "Steampipe plugin for querying Alibaba Cloud servers, databases, networks, and other resources.",
            "https://github.com/turbot/steampipe-plugin-alicloud",
        ),
        SteampipeExtension::new(
            "ansible",
            "Steampipe plugin to query configurations from the Ansible playbooks.",
            "https://github.com/turbot/steampipe-plugin-ansible",
        ),
        SteampipeExtension::new(
            "auth0",
            "Use SQL to query users, clients, connections, keys and more from Auth0.",
            "https://github.com/turbot/steampipe-plugin-auth0",
        ),
        SteampipeExtension::new(
            "aws",
            "Steampipe plugin for querying instances, buckets, databases and more from AWS.",
            "https://github.com/turbot/steampipe-plugin-aws",
        ),
        SteampipeExtension::new(
            "awscfn",
            "Steampipe plugin to query data from AWS CloudFormation template files.",
            "https://github.com/turbot/steampipe-plugin-awscfn",
        ),
        SteampipeExtension::new(
            "azure",
            "Steampipe plugin for querying resource groups, virtual machines, storage accounts and more from Azure.",
            "https://github.com/turbot/steampipe-plugin-azure",
        ),
        SteampipeExtension::new(
            "azuread",
            "Steampipe plugin for querying resource users, groups, applications and more from Azure Active Directory.",
            "https://github.com/turbot/steampipe-plugin-azuread",
        ),
        SteampipeExtension::new(
            "azuredevops",
            "Steampipe plugin to query projects, groups, builds and more from Azure DevOps.",
            "https://github.com/turbot/steampipe-plugin-azuredevops",
        ),
        SteampipeExtension::new(
            "baleen",
            "Steampipe plugin for querying Baleen.",
            "https://github.com/francois2metz/steampipe-plugin-baleen",
        ),
        SteampipeExtension::new(
            "bitbucket",
            "Steampipe plugin for querying repositories, issues, pull requests and more from Bitbucket.",
            "https://github.com/turbot/steampipe-plugin-bitbucket",
        ),
        SteampipeExtension::new(
            "bitfinex",
            "Steampipe plugin for querying data from bitfinex",
            "https://github.com/kaggrwal/steampipe-plugin-bitfinex",
        ),
        SteampipeExtension::new(
            "btp",
            "Steampipe plugin to query the account details of your SAP Business Technology Platform account.",
            "https://github.com/ajmaradiaga/steampipe-plugin-btp",
        ),
        SteampipeExtension::new(
            "buildkite",
            "Steampipe plugin to query Buildkite pipelines, builds, users and more.",
            "https://github.com/turbot/steampipe-plugin-buildkite",
        ),
        SteampipeExtension::new(
            "chaos",
            "Steampipe plugin to cause chaos for testing Steampipe with the craziest edge cases we can think of.",
            "https://github.com/turbot/steampipe-plugin-chaos",
        ),
        SteampipeExtension::new(
            "chaosdynamic",
            "Steampipe plugin to test aggregation of dynamic plugin connections.",
            "https://github.com/turbot/steampipe-plugin-chaosdynamic",
        ),
        SteampipeExtension::new(
            "circleci",
            "Steampipe plugin for querying resource projects, pipelines, builds and more from CircleCI.",
            "https://github.com/turbot/steampipe-plugin-circleci",
        ),
        SteampipeExtension::new(
            "clickup",
            "Steampipe plugin for querying ClickUp Tasks, Lists and other resources.",
            "https://github.com/theapsgroup/steampipe-plugin-clickup",
        ),
        SteampipeExtension::new(
            "cloudflare",
            "Steampipe plugin for querying Cloudflare databases, networks, and other resources.",
            "https://github.com/turbot/steampipe-plugin-cloudflare",
        ),
        SteampipeExtension::new(
            "code",
            "Steampipe plugin to query secrets and more from Code.",
            "https://github.com/turbot/steampipe-plugin-code",
        ),
        SteampipeExtension::new(
            "cohereai",
            "Steampipe plugin to query generations, classifications and more from CohereAI.",
            "https://github.com/mr-destructive/steampipe-plugin-cohereai",
        ),
        SteampipeExtension::new(
            "config",
            "Steampipe plugin to query data from various types of files like INI, JSON, YML and more.",
            "https://github.com/turbot/steampipe-plugin-config",
        ),
        SteampipeExtension::new(
            "confluence",
            "Steampipe plugin for querying pages, spaces, and more from Confluence.",
            "https://github.com/ellisvalentiner/steampipe-plugin-confluence",
        ),
        SteampipeExtension::new(
            "consul",
            "Steampipe plugin to query nodes, ACLs, services and more from Consul.",
            "https://github.com/turbot/steampipe-plugin-consul",
        ),
        SteampipeExtension::new(
            "crowdstrike",
            "Steampipe plugin to query resources from CrowdStrike.",
            "https://github.com/turbot/steampipe-plugin-crowdstrike",
        ),
        SteampipeExtension::new(
            "crtsh",
            "Steampipe plugin to query certificates, logs and more from the crt.sh certificate transparency database.",
            "https://github.com/turbot/steampipe-plugin-crtsh",
        ),
        SteampipeExtension::new(
            "csv",
            "Steampipe plugin to query data from CSV files.",
            "https://github.com/turbot/steampipe-plugin-csv",
        ),
        SteampipeExtension::new(
            "databricks",
            "Steampipe plugin to query clusters, jobs, users, and more from Databricks.",
            "https://github.com/turbot/steampipe-plugin-databricks",
        ),
        SteampipeExtension::new(
            "datadog",
            "Steampipe plugin for querying dashboards, users, roles and more from Datadog.",
            "https://github.com/turbot/steampipe-plugin-datadog",
        ),
        SteampipeExtension::new(
            "digitalocean",
            "Steampipe plugin for querying DigitalOcean databases, networks, and other resources.",
            "https://github.com/turbot/steampipe-plugin-digitalocean",
        ),
        SteampipeExtension::new(
            "docker",
            "Steampipe plugin to query Dockerfile commands and more from Docker.",
            "https://github.com/turbot/steampipe-plugin-docker",
        ),
        SteampipeExtension::new(
            "dockerhub",
            "Steampipe plugin for querying Docker Hub repositories, tags and other resources.",
            "https://github.com/turbot/steampipe-plugin-dockerhub",
        ),
        SteampipeExtension::new(
            "doppler",
            "Steampipe plugin to query projects, environments, secrets and more from Doppler.",
            "https://github.com/turbot/steampipe-plugin-doppler",
        ),
        SteampipeExtension::new(
            "duo",
            "Steampipe plugin for querying Duo Security users, logs and more.",
            "https://github.com/turbot/steampipe-plugin-duo",
        ),
        SteampipeExtension::new(
            "env0",
            "Steampipe plugin to query projects, teams, users and more from env0.",
            "https://github.com/turbot/steampipe-plugin-env0",
        ),
        SteampipeExtension::new(
            "equinix",
            "Steampipe plugin for querying Equinix Metal servers, networks, facilities and more.",
            "https://github.com/turbot/steampipe-plugin-equinix",
        ),
        SteampipeExtension::new(
            "exec",
            "Steampipe plugin to run & query shell commands on local and remote servers.",
            "https://github.com/turbot/steampipe-plugin-exec",
        ),
        SteampipeExtension::new(
            "fastly",
            "Steampipe plugin to query services, acls, domains and more from Fastly.",
            "https://github.com/turbot/steampipe-plugin-fastly",
        ),
        SteampipeExtension::new(
            "finance",
            "Steampipe plugin to query financial data including quotes and public company information.",
            "https://github.com/turbot/steampipe-plugin-finance",
        ),
        SteampipeExtension::new(
            "fly",
            "Steampipe plugin to query applications, volumes, databases, and more from your Fly organization.",
            "https://github.com/turbot/steampipe-plugin-fly",
        ),
        SteampipeExtension::new(
            "freshping",
            "Steampipe plugin for querying Freshping.",
            "https://github.com/francois2metz/steampipe-plugin-freshping",
        ),
        SteampipeExtension::new(
            "freshservice",
            "Steampipe plugin for querying FreshService agents, assets, tickets and other resources.",
            "https://github.com/theapsgroup/steampipe-plugin-freshservice",
        ),
        SteampipeExtension::new(
            "gandi",
            "Steampipe plugin for querying domains, mailboxes, certificates and more from Gandi.",
            "https://github.com/francois2metz/steampipe-plugin-gandi",
        ),
        SteampipeExtension::new(
            "gcp",
            "Steampipe plugin for querying buckets, instances, functions and more from GCP.",
            "https://github.com/turbot/steampipe-plugin-gcp",
        ),
        SteampipeExtension::new(
            "gitguardian",
            "Steampipe plugin for querying incidents from GitGuardian.",
            "https://github.com/francois2metz/steampipe-plugin-gitguardian",
        ),
        SteampipeExtension::new(
            "github",
            "Steampipe plugin for querying GitHub Repositories, Organizations, and other resources.",
            "https://github.com/turbot/steampipe-plugin-github",
        ),
        SteampipeExtension::new(
            "gitlab",
            "Steampipe plugin for querying GitLab Repositories, Users and other resources.",
            "https://github.com/theapsgroup/steampipe-plugin-gitlab",
        ),
        SteampipeExtension::new(
            "godaddy",
            "Steampipe plugin to query domains, orders, certificates and more from GoDaddy.",
            "https://github.com/turbot/steampipe-plugin-godaddy",
        ),
        SteampipeExtension::new(
            "googledirectory",
            "Steampipe plugin for querying users, groups, org units and more from your Google Workspace directory.",
            "https://github.com/turbot/steampipe-plugin-googledirectory",
        ),
        SteampipeExtension::new(
            "googlesearchconsole",
            "Steampipe plugin for query data from Google Search Console (GSC).",
            "https://github.com/turbot/steampipe-plugin-googlesearchconsole",
        ),
        SteampipeExtension::new(
            "googlesheets",
            "Steampipe plugin for query data from Google Sheets.",
            "https://github.com/turbot/steampipe-plugin-googlesheets",
        ),
        SteampipeExtension::new(
            "googleworkspace",
            "Steampipe plugin for querying users, groups, org units and more from your Google Workspace.",
            "https://github.com/turbot/steampipe-plugin-googleworkspace",
        ),
        SteampipeExtension::new(
            "grafana",
            "Steampipe plugin to query dashboards, data sources and more from Grafana.",
            "https://github.com/turbot/steampipe-plugin-grafana",
        ),
        SteampipeExtension::new(
            "guardrails",
            "Steampipe plugin to query resources, controls, policies and more from Turbot Guardrails.",
            "https://github.com/turbot/steampipe-plugin-guardrails",
        ),
        SteampipeExtension::new(
            "hackernews",
            "Steampipe plugin to query stories, items and users from Hacker News.",
            "https://github.com/turbot/steampipe-plugin-hackernews",
        ),
        SteampipeExtension::new(
            "hcloud",
            "Steampipe plugin to query servers, networks and more from Hetzner Cloud.",
            "https://github.com/turbot/steampipe-plugin-hcloud",
        ),
        SteampipeExtension::new(
            "heroku",
            "Steampipe plugin to query apps, dynos and more from Heroku.",
            "https://github.com/turbot/steampipe-plugin-heroku",
        ),
        SteampipeExtension::new(
            "hibp",
            "Steampipe plugin to query breaches, account breaches, pastes and passwords from Have I Been Pwned.",
            "https://github.com/turbot/steampipe-plugin-hibp",
        ),
        SteampipeExtension::new(
            "hubspot",
            "Steampipe plugin to query contacts, deals, tickets and more from HubSpot.",
            "https://github.com/turbot/steampipe-plugin-hubspot",
        ),
        SteampipeExtension::new(
            "hypothesis",
            "Steampipe plugin to query Hypothesis annotations.",
            "https://github.com/turbot/steampipe-plugin-hypothesis",
        ),
        SteampipeExtension::new(
            "ibm",
            "Steampipe plugin to query resources, users and more from IBM Cloud.",
            "https://github.com/turbot/steampipe-plugin-ibm",
        ),
        SteampipeExtension::new(
            "imap",
            "Steampipe plugin to query mailboxes and messages using IMAP.",
            "https://github.com/turbot/steampipe-plugin-imap",
        ),
        SteampipeExtension::new(
            "ip2locationio",
            "Steampipe plugin to query IP geolocation or WHOIS information from ip2location.io.",
            "https://github.com/ip2location/steampipe-plugin-ip2locationio",
        ),
        SteampipeExtension::new(
            "ipinfo",
            "Steampipe plugin to query IP address information from ipinfo.io.",
            "https://github.com/turbot/steampipe-plugin-ipinfo",
        ),
        SteampipeExtension::new(
            "ipstack",
            "Steampipe plugin for querying location, currency, timezone and security information about an IP address from ipstack.",
            "https://github.com/turbot/steampipe-plugin-ipstack",
        ),
        SteampipeExtension::new(
            "jenkins",
            "Steampipe plugin for querying resource jobs, builds, nodes, plugin and more from Jenkins.",
            "https://github.com/turbot/steampipe-plugin-jenkins",
        ),
        SteampipeExtension::new(
            "jira",
            "Steampipe plugin for querying sprints, issues, epics and more from Jira.",
            "https://github.com/turbot/steampipe-plugin-jira",
        ),
        SteampipeExtension::new(
            "jumpcloud",
            "Steampipe plugin to query servers, applications, user groups, and more from your JumpCloud organization.",
            "https://github.com/turbot/steampipe-plugin-jumpcloud",
        ),
        SteampipeExtension::new(
            "keycloak",
            "Steampipe plugin for querying Keycloak users, groups and other resources.",
            "https://github.com/theapsgroup/steampipe-plugin-keycloak",
        ),
        SteampipeExtension::new(
            "kolide",
            "Kolide gives you accurate, valuable and complete fleet visibility across Mac, Windows and Linux endpoints",
            "https://github.com/grendel-consulting/steampipe-plugin-kolide",
        ),
        SteampipeExtension::new(
            "kubernetes",
            "Steampipe plugin for Kubernetes components.",
            "https://github.com/turbot/steampipe-plugin-kubernetes",
        ),
        SteampipeExtension::new(
            "launchdarkly",
            "Steampipe plugin to query projects, teams, metrics, flags and more from LaunchDarkly.",
            "https://github.com/turbot/steampipe-plugin-launchdarkly",
        ),
        SteampipeExtension::new(
            "ldap",
            "Steampipe plugin for querying users, groups, organizational units and more from LDAP.",
            "https://github.com/turbot/steampipe-plugin-ldap",
        ),
        SteampipeExtension::new(
            "linear",
            "Steampipe plugin to query issues, teams, users and more from Linear.",
            "https://github.com/turbot/steampipe-plugin-linear",
        ),
        SteampipeExtension::new(
            "linkedin",
            "Steampipe plugin to query LinkedIn profiles.",
            "https://github.com/turbot/steampipe-plugin-linkedin",
        ),
        SteampipeExtension::new(
            "linode",
            "Steampipe plugin to query resources, users and more from Linode.",
            "https://github.com/turbot/steampipe-plugin-linode",
        ),
        SteampipeExtension::new(
            "mailchimp",
            "Steampipe plugin to query audiences, automation workflows, campaigns, and more from Mailchimp.",
            "https://github.com/turbot/steampipe-plugin-mailchimp",
        ),
        SteampipeExtension::new(
            "make",
            "Make plugin for exploring your automations in depth.",
            "https://github.com/marekjalovec/steampipe-plugin-make",
        ),
        SteampipeExtension::new(
            "mastodon",
            "Use SQL to instantly query Mastodon timelines, accounts, followers and more.",
            "https://github.com/turbot/steampipe-plugin-mastodon",
        ),
        SteampipeExtension::new(
            "microsoft365",
            "Steampipe plugin for querying calendars, contacts, drives, mailboxes and more from Microsoft 365.",
            "https://github.com/turbot/steampipe-plugin-microsoft365",
        ),
        SteampipeExtension::new(
            "mongodbatlas",
            "Steampipe plugin for querying clusters, users, teams and more from MongoDB Atlas.",
            "https://github.com/turbot/steampipe-plugin-mongodbatlas",
        ),
        SteampipeExtension::new(
            "namecheap",
            "Steampipe plugin to query domains, DNS host records and more from Namecheap.",
            "https://github.com/turbot/steampipe-plugin-namecheap",
        ),
        SteampipeExtension::new(
            "net",
            "Steampipe plugin for querying DNS records, certificates and other network information.",
            "https://github.com/turbot/steampipe-plugin-net",
        ),
        SteampipeExtension::new(
            "newrelic",
            "Steampipe plugin for querying New Relic Alerts, Events and other resources.",
            "https://github.com/turbot/steampipe-plugin-newrelic",
        ),
        SteampipeExtension::new(
            "nomad",
            "Steampipe plugin to query nodes, jobs, deployments and more from Nomad.",
            "https://github.com/turbot/steampipe-plugin-nomad",
        ),
        SteampipeExtension::new(
            "oci",
            "Steampipe plugin for Oracle Cloud Infrastructure services and resource types.",
            "https://github.com/turbot/steampipe-plugin-oci",
        ),
        SteampipeExtension::new(
            "okta",
            "Steampipe plugin for querying resource users, groups, applications and more from Okta.",
            "https://github.com/turbot/steampipe-plugin-okta",
        ),
        SteampipeExtension::new(
            "onepassword",
            "Steampipe plugin to query vaults, items, files and more from 1Password.",
            "https://github.com/turbot/steampipe-plugin-onepassword",
        ),
        SteampipeExtension::new(
            "openai",
            "Steampipe plugin to query models, completions and more from OpenAI.",
            "https://github.com/turbot/steampipe-plugin-openai",
        ),
        SteampipeExtension::new(
            "openapi",
            "Steampipe plugin to query introspection of the OpenAPI definition.",
            "https://github.com/turbot/steampipe-plugin-openapi",
        ),
        SteampipeExtension::new(
            "openshift",
            "Steampipe plugin to query projects, routes, builds and more from OpenShift.",
            "https://github.com/turbot/steampipe-plugin-openshift",
        ),
        SteampipeExtension::new(
            "openstack",
            "Steampipe plugin to query cloud resource information from OpenStack deployments.",
            "https://github.com/ernw/steampipe-plugin-openstack",
        ),
        SteampipeExtension::new(
            "opsgenie",
            "Steampipe plugin for querying teams and alerts from Opsgenie.",
            "https://github.com/jplanckeel/steampipe-plugin-opsgenie",
        ),
        SteampipeExtension::new(
            "ovh",
            "Steampipe plugin for querying OVH.",
            "https://github.com/francois2metz/steampipe-plugin-ovh",
        ),
        SteampipeExtension::new(
            "pagerduty",
            "Steampipe plugin to query services, teams, escalation policies and more from your PagerDuty account.",
            "https://github.com/turbot/steampipe-plugin-pagerduty",
        ),
        SteampipeExtension::new(
            "panos",
            "Steampipe plugin to query PAN-OS firewalls, security policies and more.",
            "https://github.com/turbot/steampipe-plugin-panos",
        ),
        SteampipeExtension::new(
            "pipes",
            "Steampipe plugin for querying workspaces, connections and more from Turbot Pipes.",
            "https://github.com/turbot/steampipe-plugin-pipes",
        ),
        SteampipeExtension::new(
            "planetscale",
            "Steampipe plugin to query databases, logs and more from PlanetScale.",
            "https://github.com/turbot/steampipe-plugin-planetscale",
        ),
        SteampipeExtension::new(
            "prometheus",
            "Steampipe plugin to query metrics, labels, alerts and more from Prometheus.",
            "https://github.com/turbot/steampipe-plugin-prometheus",
        ),
        SteampipeExtension::new(
            "reddit",
            "Steampipe plugin to query Reddit users, posts, votes and more.",
            "https://github.com/turbot/steampipe-plugin-reddit",
        ),
        SteampipeExtension::new(
            "rss",
            "Steampipe plugin to query RSS channels & Atom feeds",
            "https://github.com/turbot/steampipe-plugin-rss",
        ),
        SteampipeExtension::new(
            "salesforce",
            "Steampipe plugin to query accounts, opportunities, users and more from your Salesforce instance.",
            "https://github.com/turbot/steampipe-plugin-salesforce",
        ),
        SteampipeExtension::new(
            "scaleway",
            "Steampipe plugin to query servers, networks, databases and more from your Scaleway project.",
            "https://github.com/turbot/steampipe-plugin-scaleway",
        ),
        SteampipeExtension::new(
            "scalingo",
            "Steampipe plugin for querying apps, addons and more from Scalingo.",
            "https://github.com/francois2metz/steampipe-plugin-scalingo",
        ),
        SteampipeExtension::new(
            "semgrep",
            "Steampipe plugin to query deployments, findings, and projects from Semgrep.",
            "https://github.com/gabrielsoltz/steampipe-plugin-semgrep",
        ),
        SteampipeExtension::new(
            "sentry",
            "Steampipe plugin to query organizations, projects, teams and more from Sentry.",
            "https://github.com/turbot/steampipe-plugin-sentry",
        ),
        SteampipeExtension::new(
            "servicenow",
            "Use SQL to query CMDB CI services, servers, incidents, objects and more from ServiceNow.",
            "https://github.com/turbot/steampipe-plugin-servicenow",
        ),
        SteampipeExtension::new(
            "shodan",
            "Steampipe plugin to query host, DNS and exploit information using Shodan.",
            "https://github.com/turbot/steampipe-plugin-shodan",
        ),
        SteampipeExtension::new(
            "shopify",
            "Steampipe plugin to query products, order, customers and more from Shopify.",
            "https://github.com/turbot/steampipe-plugin-shopify",
        ),
        SteampipeExtension::new(
            "slack",
            "Steampipe plugin for querying Slack Conversations, Groups, Users and other resources.",
            "https://github.com/turbot/steampipe-plugin-slack",
        ),
        SteampipeExtension::new(
            "snowflake",
            "Steampipe plugin for querying roles, databases, and more from Snowflake.",
            "https://github.com/turbot/steampipe-plugin-snowflake",
        ),
        SteampipeExtension::new(
            "solace",
            "Solace PubSub+ Cloud plugin for exploring your Solace Cloud configuration in depth.",
            "https://github.com/solacelabs/steampipe-plugin-solace",
        ),
        SteampipeExtension::new(
            "splunk",
            "Steampipe plugin to query apps, indexes, logs and more from Splunk.",
            "https://github.com/turbot/steampipe-plugin-splunk",
        ),
        SteampipeExtension::new(
            "steampipe",
            "Steampipe plugin for querying Steampipe components, such as the available plugins in the steampipe hub.",
            "https://github.com/turbot/steampipe-plugin-steampipe",
        ),
        SteampipeExtension::new(
            "steampipecloud",
            "Steampipe plugin for querying workspaces, connections and more from Steampipe Cloud.",
            "https://github.com/turbot/steampipe-plugin-steampipecloud",
        ),
        SteampipeExtension::new(
            "stripe",
            "Steampipe plugin for querying customers, products, invoices and more from Stripe.",
            "https://github.com/turbot/steampipe-plugin-stripe",
        ),
        SteampipeExtension::new(
            "supabase",
            "Steampipe plugin to query projects, functions, network restrictions, and more from your Supabase organization.",
            "https://github.com/turbot/steampipe-plugin-supabase",
        ),
        SteampipeExtension::new(
            "tailscale",
            "Steampipe plugin to query VPN networks, devices and more from tailscale.",
            "https://github.com/turbot/steampipe-plugin-tailscale",
        ),
        SteampipeExtension::new(
            "terraform",
            "Steampipe plugin to query data from Terraform files.",
            "https://github.com/turbot/steampipe-plugin-terraform",
        ),
        SteampipeExtension::new(
            "tfe",
            "Steampipe plugin to query resources, users and more from Terraform Enterprise.",
            "https://github.com/turbot/steampipe-plugin-tfe",
        ),
        SteampipeExtension::new(
            "tomba",
            "Steampipe plugin to query Domain or Email information from tomba.io.",
            "https://github.com/tomba-io/steampipe-plugin-tomba",
        ),
        SteampipeExtension::new(
            "trello",
            "Steampipe plugin to query boards, cards, lists, and more from Trello.",
            "https://github.com/turbot/steampipe-plugin-trello",
        ),
        SteampipeExtension::new(
            "trivy",
            "Steampipe plugin using Trivy to query advisories, vulnerabilities for containers, code and more.",
            "https://github.com/turbot/steampipe-plugin-trivy",
        ),
        SteampipeExtension::new(
            "turbot",
            "Steampipe plugin to query resources, controls, policies and more from Turbot.",
            "https://github.com/turbot/steampipe-plugin-turbot",
        ),
        SteampipeExtension::new(
            "twilio",
            "Steampipe plugin to query calls, messages and other communication functions from your Twilio project.",
            "https://github.com/turbot/steampipe-plugin-twilio",
        ),
        SteampipeExtension::new(
            "twitter",
            "Steampipe plugin to query tweets, users and followers from Twitter.",
            "https://github.com/turbot/steampipe-plugin-twitter",
        ),
        SteampipeExtension::new(
            "updown",
            "Steampipe plugin for querying updown.io checks, metrics and downtime data.",
            "https://github.com/turbot/steampipe-plugin-updown",
        ),
        SteampipeExtension::new(
            "uptimerobot",
            "Steampipe plugin to query monitors, alert contacts and more from UptimeRobot.",
            "https://github.com/turbot/steampipe-plugin-uptimerobot",
        ),
        SteampipeExtension::new(
            "urlscan",
            "Steampipe plugin to query URL scanning results including requests cookies, headers and more from urlscan.io.",
            "https://github.com/turbot/steampipe-plugin-urlscan",
        ),
        SteampipeExtension::new(
            "vanta",
            "Steampipe plugin to query users, policies, compliances, and more from your Vanta organization.",
            "https://github.com/turbot/steampipe-plugin-vanta",
        ),
        SteampipeExtension::new(
            "vault",
            "Steampipe plugin for querying available secret keys (not values), etc from Hashicorp Vault.",
            "https://github.com/theapsgroup/steampipe-plugin-vault",
        ),
        SteampipeExtension::new(
            "vercel",
            "Steampipe plugin to query projects, teams, domains and more from Vercel.",
            "https://github.com/turbot/steampipe-plugin-vercel",
        ),
        SteampipeExtension::new(
            "virustotal",
            "Steampipe plugin to query file, domain, URL and IP scanning results from VirusTotal.",
            "https://github.com/turbot/steampipe-plugin-virustotal",
        ),
        SteampipeExtension::new(
            "vsphere",
            "Steampipe plugin for querying data from a vsphere environment.",
            "https://github.com/theapsgroup/steampipe-plugin-vsphere",
        ),
        SteampipeExtension::new(
            "weatherkit",
            "Steampipe plugin for querying weather from WeatherKit.",
            "https://github.com/ellisvalentiner/steampipe-plugin-weatherkit",
        ),
        SteampipeExtension::new(
            "whois",
            "Steampipe plugin for querying domains, name servers and contact information from WHOIS.",
            "https://github.com/turbot/steampipe-plugin-whois",
        ),
        SteampipeExtension::new(
            "wiz",
            "Steampipe plugin to query security controls, findings, vulnerabilities, and more from your Wiz subscription.",
            "https://github.com/turbot/steampipe-plugin-wiz",
        ),
        SteampipeExtension::new(
            "workos",
            "Steampipe plugin to query directories, groups and more from WorkOS.",
            "https://github.com/turbot/steampipe-plugin-workos",
        ),
        SteampipeExtension::new(
            "zendesk",
            "Steampipe plugin for querying tickets, users, groups and more from Zendesk.",
            "https://github.com/turbot/steampipe-plugin-zendesk",
        ),
        SteampipeExtension::new(
            "zoom",
            "Steampipe plugin for querying Zoom meetings, webinars, users and more.",
            "https://github.com/turbot/steampipe-plugin-zoom",
        ),
    ]
}

#[derive(Debug)]
pub struct SteampipeExtension {
    pub name: String,
    pub description: String,
    pub url: String,
}

impl SteampipeExtension {
    pub fn new(name: &str, description: &str, url: &str) -> SteampipeExtension {
        SteampipeExtension {
            name: name.to_string(),
            description: description.to_string(),
            url: url.to_string(),
        }
    }
}

pub fn get<'a>() -> &'a Vec<SteampipeExtension> {
    &EXTENSIONS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let extensions = get();
        assert_eq!(143, extensions.len());
    }
}
