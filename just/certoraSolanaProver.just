# expects that the following variables are defined
# project_root -- root of the project
# create_root -- root of the crate

# certora endpoint
export CERTORA_SOLANA_CLI := env_var_or_default("CERTORA_SOLANA_CLI", "certoraSolanaProver")

certora_root := project_root / "certora"
certora_summaries := certora_root / "summaries"
core_inliner_cfg := certora_summaries  / "cvlr_inlining_core.txt"
core_summaries_cfg := certora_summaries  / "cvlr_summaries_core.txt"


inliner_cfg := crate_root / "certora" / "cvlr_inlining.txt"
summaries_cfg := crate_root / "certora" / "cvlr_summaries.txt"



# recipes for running certoraSolanaProver
confs_justfile := crate_root / "certora/confs/justfile"

make-certora-sbf-env:
	echo ""
	# disabled because simple demo has no dynamic env configurations
	# cat {{ core_inliner_cfg }} > {{ inliner_cfg }}
	# cat {{ core_summaries_cfg }} > {{ summaries_cfg }}
	
certora RULE: make-certora-sbf-env
	just -f {{ confs_justfile }} certora {{ RULE }}

certora-conf CONF *OPTS: make-certora-sbf-env
	cd {{ parent_dir(CONF) }} && ${CERTORA_SOLANA_CLI} {{ file_name(CONF) }} {{ OPTS }}


deep-clean: clean
	just -f {{ confs_justfile }} clean




