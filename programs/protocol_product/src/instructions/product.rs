use crate::error::ProductError;
use crate::state::product::Product;
use anchor_lang::prelude::*;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

// create product/protocol configuration account
pub fn create_product(
    product: &mut Product,
    authority: &Pubkey,
    payer: &Pubkey,
    product_title: String,
    commission_rate: f64,
    commission_escrow: &Pubkey,
) -> Result<()> {
    validate_commission_rate(commission_rate)?;

    require!(
        !&product_title.trim().is_empty(),
        ProductError::ProductTitleEmpty
    );

    product.authority = *authority;
    product.payer = *payer;
    product.product_title = product_title;
    product.commission_rate = commission_rate;
    product.commission_escrow = *commission_escrow;
    Ok(())
}

pub fn update_product_commission_escrow(
    product: &mut Product,
    updated_commission_escrow: Pubkey,
) -> Result<()> {
    product.commission_escrow = updated_commission_escrow;
    Ok(())
}

pub fn update_product_commission_rate(
    product: &mut Product,
    updated_commission_rate: f64,
) -> Result<()> {
    validate_commission_rate(updated_commission_rate)?;

    product.commission_rate = updated_commission_rate;
    Ok(())
}

pub fn update_product_authority(product: &mut Product, updated_authority: Pubkey) -> Result<()> {
    product.authority = updated_authority;
    Ok(())
}

fn validate_commission_rate(commission_rate: f64) -> Result<()> {
    require!(
        (0.0..=100.0).contains(&commission_rate),
        ProductError::InvalidCommissionRate
    );

    let decimal = Decimal::from_f64(commission_rate).unwrap();
    let decimal_with_scale = decimal.trunc_with_scale(3);
    require!(
        decimal.eq(&decimal_with_scale),
        ProductError::CommissionPrecisionTooLarge
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_response() {
        let mut empty_product_account = Product {
            authority: Default::default(),
            payer: Default::default(),
            commission_escrow: Default::default(),
            product_title: "".to_string(),
            commission_rate: 0.0,
        };

        let result = create_product(
            &mut empty_product_account,
            &Pubkey::new_unique(),
            &Pubkey::new_unique(),
            "TITLE".to_string(),
            1.1,
            &Pubkey::new_unique(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_commission_rate_precision_too_large() {
        let mut empty_product_account = Product {
            authority: Default::default(),
            payer: Default::default(),
            commission_escrow: Default::default(),
            product_title: "".to_string(),
            commission_rate: 0.0,
        };

        let result = create_product(
            &mut empty_product_account,
            &Pubkey::new_unique(),
            &Pubkey::new_unique(),
            "TITLE".to_string(),
            1.1111,
            &Pubkey::new_unique(),
        );
        let expected_error = Err(error!(ProductError::CommissionPrecisionTooLarge));
        assert_eq!(expected_error, result);
    }

    #[test]
    fn test_invalid_commission_rate() {
        let mut empty_product_account = Product {
            authority: Default::default(),
            payer: Default::default(),
            commission_escrow: Default::default(),
            product_title: "".to_string(),
            commission_rate: 0.0,
        };

        let result = create_product(
            &mut empty_product_account,
            &Pubkey::new_unique(),
            &Pubkey::new_unique(),
            "TITLE".to_string(),
            101.11,
            &Pubkey::new_unique(),
        );
        let expected_error = Err(error!(ProductError::InvalidCommissionRate));
        assert_eq!(expected_error, result);
    }

    #[test]
    fn test_title_must_not_be_empty_validation() {
        let mut empty_product_account = Product {
            authority: Default::default(),
            payer: Default::default(),
            commission_escrow: Default::default(),
            product_title: "".to_string(),
            commission_rate: 0.0,
        };

        let result = create_product(
            &mut empty_product_account,
            &Pubkey::new_unique(),
            &Pubkey::new_unique(),
            "    ".to_string(),
            99.99,
            &Pubkey::new_unique(),
        );
        let expected_error = Err(error!(ProductError::ProductTitleEmpty));
        assert_eq!(expected_error, result);

        let result = create_product(
            &mut empty_product_account,
            &Pubkey::new_unique(),
            &Pubkey::new_unique(),
            "".to_string(),
            99.99,
            &Pubkey::new_unique(),
        );
        let expected_error = Err(error!(ProductError::ProductTitleEmpty));
        assert_eq!(expected_error, result);
    }

    #[test]
    fn test_update_commission_rate_ok_result() {
        let mut product = Product {
            authority: Default::default(),
            payer: Default::default(),
            commission_escrow: Default::default(),
            product_title: "".to_string(),
            commission_rate: 0.0,
        };

        let result = update_product_commission_rate(&mut product, 99.99);

        assert!(result.is_ok());
        assert_eq!(product.commission_rate, 99.99)
    }

    #[test]
    fn test_update_commission_rate_invalid_commission_rate() {
        let mut product = Product {
            authority: Default::default(),
            payer: Default::default(),
            commission_escrow: Default::default(),
            product_title: "".to_string(),
            commission_rate: 0.0,
        };

        let result = update_product_commission_rate(&mut product, 199.99);

        let expected_error = Err(error!(ProductError::InvalidCommissionRate));
        assert_eq!(expected_error, result);
    }

    #[test]
    fn test_update_commission_rate_precision_too_large() {
        let mut product = Product {
            authority: Default::default(),
            payer: Default::default(),
            commission_escrow: Default::default(),
            product_title: "".to_string(),
            commission_rate: 0.0,
        };

        let result = update_product_commission_rate(&mut product, 99.9999);

        let expected_error = Err(error!(ProductError::CommissionPrecisionTooLarge));
        assert_eq!(expected_error, result);
    }

    #[test]
    fn test_update_commission_escrow_ok_result() {
        let mut product = Product {
            authority: Default::default(),
            payer: Default::default(),
            commission_escrow: Default::default(),
            product_title: "".to_string(),
            commission_rate: 0.0,
        };

        let new_escrow = Pubkey::new_unique();
        let result = update_product_commission_escrow(&mut product, new_escrow);

        assert!(result.is_ok());
        assert_eq!(product.commission_escrow, new_escrow)
    }
}
