// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

//! Extension utilities for creating derived user types.
//!
//! This module provides the [`extend_user!`] macro for creating
//! application-specific user types that extend [`RUser`] with
//! custom fields while preserving all base functionality.
//!
//! # Overview
//!
//! The [`extend_user!`] macro generates:
//!
//! | Generated | Description |
//! |-----------|-------------|
//! | Struct | Your custom type with embedded `RUser` |
//! | Builder | Type-safe builder via `bon` |
//! | `Deref`/`DerefMut` | Transparent access to `RUser` fields |
//! | `From<T> for RUser` | Convert back to base type |
//! | `From<T> for RUserPublic` | Direct projection conversion |
//! | Constructors | `from_telegram`, `from_email`, `from_phone` |
//! | Helpers | `as_user`, `to_public`, `to_auth` |
//!
//! # Features
//!
//! - **Compile-time safety**: Required fields are enforced at compile time
//! - **Transparent access**: Deref to [`RUser`] for seamless field access
//! - **Fluent API**: Builder pattern with preset constructors
//! - **Serde flatten**: JSON serialization produces flat structure
//! - **Full bon integration**: All bon builder features available
//!
//! # Basic Example
//!
//! ```rust,ignore
//! use revelation_user::{extend_user, RUser, Gender};
//! use uuid::Uuid;
//!
//! extend_user! {
//!     /// Corporate user with company-specific fields.
//!     pub struct CorpUser {
//!         /// Company identifier.
//!         pub company_id: Uuid,
//!
//!         /// Department name.
//!         #[builder(into)]
//!         pub department: String,
//!
//!         /// Manager flag.
//!         #[builder(default)]
//!         pub is_manager: bool,
//!     }
//! }
//!
//! // Fluent construction from auth method
//! let user = CorpUser::from_telegram(123456789)
//!     .company_id(Uuid::now_v7())
//!     .department("Engineering")
//!     .build();
//!
//! // Transparent field access via Deref
//! assert!(user.telegram_id.is_some());  // RUser field
//! assert!(!user.is_manager);            // CorpUser field
//! ```
//!
//! # Builder Flow
//!
//! The builder has two phases:
//!
//! ```text
//! ExtendedBuilder (RUser fields)  ->  TypeBuilder (custom fields)  ->  build()
//!        │                                    │
//!        ├── .name("...")                     ├── .company_id(...)
//!        ├── .email("...")                    ├── .department("...")
//!        ├── .gender(...)                     └── .is_manager(...)
//!        └── .then() ─────────────────────────┘
//! ```
//!
//! ## Direct Build (Skip RUser Configuration)
//!
//! ```rust,ignore
//! use revelation_user::{extend_user, RUser};
//! use uuid::Uuid;
//!
//! extend_user! {
//!     pub struct CorpUser {
//!         pub company_id: Uuid,
//!         #[builder(into)]
//!         pub department: String,
//!     }
//! }
//!
//! // Skip to custom fields immediately
//! let user = CorpUser::from_telegram(123)
//!     .company_id(Uuid::now_v7())
//!     .department("Eng")
//!     .build();
//! ```
//!
//! ## With RUser Configuration
//!
//! ```rust,ignore
//! use revelation_user::{extend_user, RUser, Gender};
//! use uuid::Uuid;
//!
//! extend_user! {
//!     pub struct CorpUser {
//!         pub company_id: Uuid,
//!         #[builder(into)]
//!         pub department: String,
//!     }
//! }
//!
//! // Configure RUser fields first, then custom fields
//! let user = CorpUser::from_telegram(123)
//!     .name("John Doe")
//!     .gender(Gender::Male)
//!     .then()  // Transition to custom fields
//!     .company_id(Uuid::now_v7())
//!     .department("Eng")
//!     .build();
//! ```
//!
//! # JSON Serialization
//!
//! Uses `#[serde(flatten)]` for flat JSON structure:
//!
//! ```rust,ignore
//! use revelation_user::{extend_user, RUser};
//! use uuid::Uuid;
//!
//! extend_user! {
//!     pub struct CorpUser {
//!         pub company_id: Uuid,
//!     }
//! }
//!
//! let user = CorpUser::from_telegram(123)
//!     .company_id(Uuid::nil())
//!     .build();
//!
//! let json = serde_json::to_string_pretty(&user).unwrap();
//! // Output (flat, not nested):
//! // {
//! //   "id": "...",
//! //   "telegram_id": 123,
//! //   "company_id": "00000000-0000-0000-0000-000000000000",
//! //   ...
//! // }
//! ```
//!
//! # Projection Conversions
//!
//! ```rust,ignore
//! use revelation_user::{extend_user, RUser, RUserPublic, RUserAuth, RUserRole};
//! use uuid::Uuid;
//!
//! extend_user! {
//!     pub struct CorpUser {
//!         pub company_id: Uuid,
//!     }
//! }
//!
//! let corp_user = CorpUser::from_telegram(123)
//!     .company_id(Uuid::now_v7())
//!     .build();
//!
//! // To public projection (excludes sensitive fields)
//! let public: RUserPublic = corp_user.to_public();
//!
//! // To auth projection (for JWT/sessions)
//! let auth: RUserAuth = corp_user.to_auth(RUserRole::Premium);
//!
//! // Direct From conversion
//! let public_direct: RUserPublic = corp_user.into();
//! ```
//!
//! [`RUser`]: crate::RUser
//! [`extend_user!`]: crate::extend_user

/// Creates an extended user type with custom fields.
///
/// This macro generates a struct that:
/// - Contains an embedded [`RUser`] with `#[serde(flatten)]`
/// - Implements `Deref` and `DerefMut` to [`RUser`]
/// - Has a bon builder with type-state pattern
/// - Provides preset constructors (`from_telegram`, `from_email`, `from_phone`)
///
/// # Syntax
///
/// ```rust,ignore
/// extend_user! {
///     /// Optional doc comments
///     #[optional_attributes]
///     pub struct TypeName {
///         /// Field doc
///         #[builder(into)]  // optional bon attributes
///         pub field_name: FieldType,
///
///         #[builder(default)]
///         pub optional_field: Option<T>,
///     }
/// }
/// ```
///
/// # Generated Methods
///
/// - `TypeName::builder()` - Full builder access
/// - `TypeName::from_telegram(id)` - Start builder from Telegram auth
/// - `TypeName::from_email(email)` - Start builder from email auth
/// - `TypeName::from_phone(phone)` - Start builder from phone auth
/// - `TypeName::from_user(RUser)` - Start builder from existing user
/// - `type_name.as_user()` - Get reference to inner RUser
/// - `type_name.as_user_mut()` - Get mutable reference to inner RUser
/// - `type_name.into_user()` - Extract inner RUser
/// - `type_name.to_public()` - Convert to RUserPublic projection
/// - `type_name.to_auth(role)` - Convert to RUserAuth projection
///
/// # Compile-Time Safety
///
/// The generated builder uses bon's type-state pattern to ensure
/// all required fields are provided at compile time:
///
/// ```rust,compile_fail
/// // This won't compile - missing required field `company_id`
/// let user = CorpUser::from_telegram(123)
///     .department("Eng")
///     .build();
/// ```
///
/// [`RUser`]: crate::RUser
#[macro_export]
macro_rules! extend_user {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::serde::Serialize,
            ::serde::Deserialize,
            ::bon::Builder
        )]
        $vis struct $name {
            /// Base user data from revelation-user.
            #[serde(flatten)]
            #[builder(into)]
            inner: $crate::RUser,

            $(
                $(#[$field_meta])*
                $field_vis $field: $ty,
            )*
        }

        impl ::core::ops::Deref for $name {
            type Target = $crate::RUser;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl ::core::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl ::core::convert::AsRef<$crate::RUser> for $name {
            #[inline]
            fn as_ref(&self) -> &$crate::RUser {
                &self.inner
            }
        }

        impl ::core::convert::AsMut<$crate::RUser> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut $crate::RUser {
                &mut self.inner
            }
        }

        impl ::core::convert::From<$name> for $crate::RUser {
            #[inline]
            fn from(ext: $name) -> Self {
                ext.inner
            }
        }

        impl ::core::convert::From<$name> for $crate::RUserPublic {
            #[inline]
            fn from(ext: $name) -> Self {
                ext.inner.into()
            }
        }

        impl $name {
            #[doc = concat!("Create [`", stringify!($name), "`] builder from Telegram authentication.")]
            ///
            /// Initializes the inner [`RUser`] with the provided Telegram ID
            /// and returns a builder for setting remaining fields.
            ///
            /// # Example
            ///
            /// ```rust,ignore
            #[doc = concat!("let user = ", stringify!($name), "::from_telegram(123456789)")]
            ///     .company_id(id)
            ///     .build();
            /// ```
            #[inline]
            #[must_use]
            pub fn from_telegram(
                telegram_id: i64
            ) -> $crate::extend::ExtendedBuilder<Self, impl FnOnce($crate::RUser) -> <Self as ::bon::Builder>::Builder> {
                $crate::extend::ExtendedBuilder::new(
                    $crate::RUser::from_telegram(telegram_id),
                    |user| Self::builder().inner(user)
                )
            }

            #[doc = concat!("Create [`", stringify!($name), "`] builder from email authentication.")]
            ///
            /// Initializes the inner [`RUser`] with the provided email
            /// and returns a builder for setting remaining fields.
            #[inline]
            #[must_use]
            pub fn from_email(
                email: impl ::core::convert::Into<String>
            ) -> $crate::extend::ExtendedBuilder<Self, impl FnOnce($crate::RUser) -> <Self as ::bon::Builder>::Builder> {
                $crate::extend::ExtendedBuilder::new(
                    $crate::RUser::from_email(email),
                    |user| Self::builder().inner(user)
                )
            }

            #[doc = concat!("Create [`", stringify!($name), "`] builder from phone authentication.")]
            ///
            /// Initializes the inner [`RUser`] with the provided phone number
            /// and returns a builder for setting remaining fields.
            #[inline]
            #[must_use]
            pub fn from_phone(
                phone: impl ::core::convert::Into<String>
            ) -> $crate::extend::ExtendedBuilder<Self, impl FnOnce($crate::RUser) -> <Self as ::bon::Builder>::Builder> {
                $crate::extend::ExtendedBuilder::new(
                    $crate::RUser::from_phone(phone),
                    |user| Self::builder().inner(user)
                )
            }

            #[doc = concat!("Create [`", stringify!($name), "`] builder from existing [`RUser`].")]
            ///
            /// Useful when you already have a user and want to extend it.
            #[inline]
            #[must_use]
            pub fn from_user(
                user: impl ::core::convert::Into<$crate::RUser>
            ) -> $crate::extend::ExtendedBuilder<Self, impl FnOnce($crate::RUser) -> <Self as ::bon::Builder>::Builder> {
                $crate::extend::ExtendedBuilder::new(
                    user.into(),
                    |user| Self::builder().inner(user)
                )
            }

            /// Get reference to the inner [`RUser`].
            #[inline]
            #[must_use]
            pub const fn as_user(&self) -> &$crate::RUser {
                &self.inner
            }

            /// Get mutable reference to the inner [`RUser`].
            #[inline]
            #[must_use]
            pub fn as_user_mut(&mut self) -> &mut $crate::RUser {
                &mut self.inner
            }

            /// Extract the inner [`RUser`], consuming self.
            #[inline]
            #[must_use]
            pub fn into_user(self) -> $crate::RUser {
                self.inner
            }

            /// Convert to public user projection.
            ///
            /// Creates an [`RUserPublic`] containing only publicly-safe fields.
            #[inline]
            #[must_use]
            pub fn to_public(&self) -> $crate::RUserPublic {
                (&self.inner).into()
            }

            /// Convert to auth user projection.
            ///
            /// Creates an [`RUserAuth`] for JWT/session context.
            #[inline]
            #[must_use]
            pub fn to_auth(&self, role: $crate::RUserRole) -> $crate::RUserAuth {
                $crate::RUserAuth::from_user(&self.inner, role)
            }
        }
    };
}

/// Builder wrapper for extended user types.
///
/// Allows method chaining to configure the inner [`RUser`]
/// before transitioning to the extended type's builder.
///
/// This is an implementation detail of [`extend_user!`] macro.
/// You typically don't create this directly - it's returned by
/// `from_telegram`, `from_email`, `from_phone`, and `from_user`.
///
/// # Builder Flow
///
/// ```text
/// ExtendedBuilder<T, F>
///     │
///     ├── .name("...")        ─┐
///     ├── .email("...")        │ Configure RUser
///     ├── .gender(...)        ─┘
///     │
///     └── .then() ───────────> TypeBuilder (bon)
///                                  │
///                                  └── .build() ───> T
/// ```
///
/// # Example
///
/// ```rust,ignore
/// // The builder lets you configure RUser fields before custom fields
/// let user = CorpUser::from_telegram(123)
///     .name("John")           // ExtendedBuilder method
///     .email("j@example.com") // ExtendedBuilder method
///     .then()                 // Transition to TypeBuilder
///     .company_id(id)         // TypeBuilder method
///     .build();
/// ```
///
/// [`RUser`]: crate::RUser
/// [`extend_user!`]: crate::extend_user
#[derive(Debug)]
pub struct ExtendedBuilder<T, F> {
    user:         crate::RUser,
    into_builder: F,
    _marker:      core::marker::PhantomData<T>
}

impl<T, F, B> ExtendedBuilder<T, F>
where
    F: FnOnce(crate::RUser) -> B
{
    /// Create a new extended builder.
    ///
    /// This is typically called by generated `from_*` methods.
    ///
    /// # Arguments
    ///
    /// * `user` - Base [`RUser`] to extend
    /// * `into_builder` - Function to transition to type-specific builder
    ///
    /// [`RUser`]: crate::RUser
    #[inline]
    #[must_use]
    pub fn new(user: crate::RUser, into_builder: F) -> Self {
        Self {
            user,
            into_builder,
            _marker: core::marker::PhantomData
        }
    }

    /// Set user display name before building.
    ///
    /// # Arguments
    ///
    /// * `name` - Display name (accepts anything implementing `Into<String>`)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let user = CorpUser::from_telegram(123)
    ///     .name("John Doe")
    ///     .company_id(id)
    ///     .build();
    /// ```
    #[inline]
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.user.name = Some(name.into());
        self
    }

    /// Set user email before building.
    ///
    /// # Arguments
    ///
    /// * `email` - Email address (accepts anything implementing `Into<String>`)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let user = CorpUser::from_telegram(123)
    ///     .email("john@example.com")
    ///     .company_id(id)
    ///     .build();
    /// ```
    #[inline]
    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.user.email = Some(email.into());
        self
    }

    /// Set user phone number before building.
    ///
    /// # Arguments
    ///
    /// * `phone` - Phone in E.164 format (e.g., `+14155551234`)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let user = CorpUser::from_telegram(123)
    ///     .phone("+14155551234")
    ///     .company_id(id)
    ///     .build();
    /// ```
    #[inline]
    #[must_use]
    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.user.phone = Some(phone.into());
        self
    }

    /// Set user gender before building.
    ///
    /// # Arguments
    ///
    /// * `gender` - User's gender ([`Gender::Male`] or [`Gender::Female`])
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use revelation_user::Gender;
    ///
    /// let user = CorpUser::from_telegram(123)
    ///     .gender(Gender::Male)
    ///     .company_id(id)
    ///     .build();
    /// ```
    ///
    /// [`Gender::Male`]: crate::Gender::Male
    /// [`Gender::Female`]: crate::Gender::Female
    #[inline]
    #[must_use]
    pub fn gender(mut self, gender: crate::Gender) -> Self {
        self.user.gender = Some(gender);
        self
    }

    /// Set user Telegram ID before building.
    ///
    /// # Arguments
    ///
    /// * `telegram_id` - Telegram user ID
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let user = CorpUser::from_email("j@example.com")
    ///     .telegram_id(123456789)  // Also link Telegram
    ///     .company_id(id)
    ///     .build();
    /// ```
    #[inline]
    #[must_use]
    pub fn telegram_id(mut self, telegram_id: i64) -> Self {
        self.user.telegram_id = Some(telegram_id);
        self
    }

    /// Finish configuring [`RUser`] and transition to custom fields builder.
    ///
    /// After calling `then()`, you'll be working with the bon-generated
    /// builder for your custom fields.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let user = CorpUser::from_telegram(123)
    ///     .name("John")
    ///     .then()              // Transition point
    ///     .company_id(id)      // Now on TypeBuilder
    ///     .department("Eng")
    ///     .build();
    /// ```
    ///
    /// [`RUser`]: crate::RUser
    #[inline]
    #[must_use]
    pub fn then(self) -> B {
        (self.into_builder)(self.user)
    }
}

/// Auto-deref to inner [`RUser`] for field inspection.
///
/// Allows read-only access to configured [`RUser`] fields
/// during the building process.
///
/// [`RUser`]: crate::RUser
impl<T, F, B> core::ops::Deref for ExtendedBuilder<T, F>
where
    F: FnOnce(crate::RUser) -> B + Clone
{
    type Target = crate::RUser;

    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Gender, RUser};

    #[test]
    fn extended_builder_new() {
        let user = RUser::from_telegram(123);
        let builder: ExtendedBuilder<(), _> = ExtendedBuilder::new(user.clone(), |u: RUser| u.id);

        assert_eq!(builder.telegram_id, Some(123));
    }

    #[test]
    fn extended_builder_name() {
        let builder: ExtendedBuilder<(), _> =
            ExtendedBuilder::new(RUser::empty(), |u: RUser| u).name("Test");

        assert_eq!(builder.user.name.as_deref(), Some("Test"));
    }

    #[test]
    fn extended_builder_email() {
        let builder: ExtendedBuilder<(), _> =
            ExtendedBuilder::new(RUser::empty(), |u: RUser| u).email("a@b.com");

        assert_eq!(builder.user.email.as_deref(), Some("a@b.com"));
    }

    #[test]
    fn extended_builder_phone() {
        let builder: ExtendedBuilder<(), _> =
            ExtendedBuilder::new(RUser::empty(), |u: RUser| u).phone("+1234567890");

        assert_eq!(builder.user.phone.as_deref(), Some("+1234567890"));
    }

    #[test]
    fn extended_builder_gender() {
        let builder: ExtendedBuilder<(), _> =
            ExtendedBuilder::new(RUser::empty(), |u: RUser| u).gender(Gender::Male);

        assert_eq!(builder.user.gender, Some(Gender::Male));
    }

    #[test]
    fn extended_builder_telegram_id() {
        let builder: ExtendedBuilder<(), _> =
            ExtendedBuilder::new(RUser::empty(), |u: RUser| u).telegram_id(999);

        assert_eq!(builder.user.telegram_id, Some(999));
    }

    #[test]
    fn extended_builder_then() {
        let result: RUser = ExtendedBuilder::<(), _>::new(RUser::from_telegram(123), |u: RUser| u)
            .name("Test")
            .then();

        assert_eq!(result.name.as_deref(), Some("Test"));
        assert_eq!(result.telegram_id, Some(123));
    }

    #[test]
    fn extended_builder_deref() {
        let builder: ExtendedBuilder<(), _> =
            ExtendedBuilder::new(RUser::from_email("x@y.com"), |u: RUser| u);

        assert_eq!(builder.email.as_deref(), Some("x@y.com"));
    }

    #[test]
    fn extended_builder_chaining() {
        let result: RUser = ExtendedBuilder::<(), _>::new(RUser::empty(), |u: RUser| u)
            .name("John")
            .email("john@example.com")
            .phone("+14155551234")
            .gender(Gender::Male)
            .telegram_id(123456789)
            .then();

        assert_eq!(result.name.as_deref(), Some("John"));
        assert_eq!(result.email.as_deref(), Some("john@example.com"));
        assert_eq!(result.phone.as_deref(), Some("+14155551234"));
        assert_eq!(result.gender, Some(Gender::Male));
        assert_eq!(result.telegram_id, Some(123456789));
    }
}
