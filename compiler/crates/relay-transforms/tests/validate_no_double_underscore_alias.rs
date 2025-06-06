/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::sync::Arc;

use common::SourceLocationKey;
use fixture_tests::Fixture;
use graphql_ir::Program;
use graphql_ir::build;
use graphql_syntax::parse_executable;
use graphql_test_helpers::diagnostics_to_sorted_string;
use relay_test_schema::TEST_SCHEMA;
use relay_transforms::validate_no_double_underscore_alias;

pub async fn transform_fixture(fixture: &Fixture<'_>) -> Result<String, String> {
    let source_location = SourceLocationKey::standalone(fixture.file_name);

    let ast = parse_executable(fixture.content, source_location).unwrap();
    let ir_result = build(&TEST_SCHEMA, &ast.definitions);
    let ir = ir_result
        .map_err(|diagnostics| diagnostics_to_sorted_string(fixture.content, &diagnostics))?;

    let program = Program::from_definitions(Arc::clone(&TEST_SCHEMA), ir);
    validate_no_double_underscore_alias(&program)
        .map_err(|diagnostics| diagnostics_to_sorted_string(fixture.content, &diagnostics))?;

    Ok("OK".to_owned())
}
