use crate::common::prompt::prompt;
use crate::config::accounts::get_accounts_from_ext;
use crate::profile_validation::profile_validation_functions::ProfileValidationStruct;
use thirtyfour::prelude::*;
pub async fn profile_validation_run(driver: WebDriver) -> WebDriverResult<()> {
    loop {
        let input = prompt()?;
        let trimmed_input = input.trim();

        if trimmed_input.to_lowercase() == "q" {
            println!("Exiting...");
            break;
        }

        let number: u32 = match trimmed_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number or 'q' to quit!");
                continue;
            }
        };

        match number {
            // Add profile
            1 => {
                println!("You entered '1'.");
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation.add_profile().await?;
            }

            2 => {
                // View Profile
                println!("You entered '2'.");
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation.view_profile().await?;
            }

            3 => {
                // Profile stake 1
                println!("You entered '3'.");
                let accounts_info = get_accounts_from_ext();
                let account_cut = accounts_info["account2"]["ss58_address"].as_str().unwrap();
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation
                    .add_profile_stake("500".to_string(), account_cut)
                    .await?;
            }
            4 => {
                // Profile stake 2
                println!("You entered '4'.");
                let accounts_info = get_accounts_from_ext();
                let account_cut = accounts_info["account3"]["ss58_address"].as_str().unwrap();
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation
                    .add_profile_stake("500".to_string(), account_cut)
                    .await?;
            }
            5 => {
                println!("You entered '5'.");
                let accounts_info = get_accounts_from_ext();
                let account_cut = accounts_info["account4"]["ss58_address"].as_str().unwrap();
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation.challenge_evidence(account_cut).await?;
            }

            6 => {
                // Staking page
                println!("You entered '6'.");
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation.schelling_game_page().await?;
            }

            7 => {
                // Juror stake 1
                println!("You entered '7'.");
                let accounts_info = get_accounts_from_ext();
                let account_stake = accounts_info["account5"]["ss58_address"].as_str().unwrap();
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation.apply_juror(account_stake, "500").await?;
            }
            8 => {
                // Juror stake 2
                println!("You entered '8'.");
                let accounts_info = get_accounts_from_ext();
                let account_stake = accounts_info["account6"]["ss58_address"].as_str().unwrap();
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation.apply_juror(account_stake, "800").await?;
            }
            9 => {
                // Juror stake 3
                println!("You entered '9'.");
                let accounts_info = get_accounts_from_ext();
                let account_stake = accounts_info["account7"]["ss58_address"].as_str().unwrap();
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation
                    .apply_juror(account_stake, "1500")
                    .await?;
            }
            10 => {
                // Juror stake 4
                println!("You entered '10'.");
                let accounts_info = get_accounts_from_ext();
                let account_stake = accounts_info["account8"]["ss58_address"].as_str().unwrap();
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation
                    .apply_juror(account_stake, "2000")
                    .await?;
            }
            11 => {
                // Juror stake 5
                println!("You entered '11'.");
                let accounts_info = get_accounts_from_ext();
                let account_stake = accounts_info["account9"]["ss58_address"].as_str().unwrap();
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation
                    .apply_juror(account_stake, "2500")
                    .await?;
            }
            12 => {
                // Juror stake 6
                println!("You entered '12'.");
                let accounts_info = get_accounts_from_ext();
                let account_stake = accounts_info["account10"]["ss58_address"].as_str().unwrap();
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation
                    .apply_juror(account_stake, "2800")
                    .await?;
            }

            13 => {
                // Change Period
                println!("You entered '13'");
                let accounts_info = get_accounts_from_ext();
                let account = accounts_info["account11"]["ss58_address"].as_str().unwrap();
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation.change_period(account).await?;
            }
            14 => {
                // Change Period
                println!("You entered '14'");
                let accounts_info = get_accounts_from_ext();
                let account_for_draw = accounts_info["account11"]["ss58_address"].as_str().unwrap();
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation
                    .draw_jurors("3", account_for_draw)
                    .await?;
            }
            15 => {
                // Change Period
                println!("You entered '15'");
                let accounts_info = get_accounts_from_ext();
                let account_for_draw = accounts_info["account11"]["ss58_address"].as_str().unwrap();
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation
                    .draw_jurors("2", account_for_draw)
                    .await?;
            }
            16 => {
                // Change Period
                println!("You entered '16'");
                let accounts_info = get_accounts_from_ext();
                let account = accounts_info["account11"]["ss58_address"].as_str().unwrap();
                let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
                profile_validation.change_period(account).await?;
            }
            _ => {
                println!("Please enter a valid number");
            }
        }
    }

    Ok(())
}
