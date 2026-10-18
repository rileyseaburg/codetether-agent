//! Command parsing regressions.
//! Tests cover aliases, argument boundaries and passthrough input.
mod command_with_optional_args_avoids_prefix_collisions;
mod normalize_easy_command_add_alias;
mod normalize_easy_command_focus_alias;
mod normalize_easy_command_home_alias;
mod normalize_easy_command_list_alias;
mod normalize_easy_command_passthrough;
mod normalize_easy_command_remove_alias;
mod normalize_easy_command_talk_alias;
mod normalize_slash_command_preserves_arguments;
