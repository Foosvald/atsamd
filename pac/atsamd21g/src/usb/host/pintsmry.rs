#[doc = "Register `PINTSMRY` reader"]
pub struct R(crate::R<PINTSMRY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINTSMRY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINTSMRY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINTSMRY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPINT0` reader - Pipe 0 Interrupt"]
pub struct EPINT0_R(crate::FieldReader<bool, bool>);
impl EPINT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPINT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPINT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPINT1` reader - Pipe 1 Interrupt"]
pub struct EPINT1_R(crate::FieldReader<bool, bool>);
impl EPINT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPINT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPINT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPINT2` reader - Pipe 2 Interrupt"]
pub struct EPINT2_R(crate::FieldReader<bool, bool>);
impl EPINT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPINT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPINT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPINT3` reader - Pipe 3 Interrupt"]
pub struct EPINT3_R(crate::FieldReader<bool, bool>);
impl EPINT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPINT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPINT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPINT4` reader - Pipe 4 Interrupt"]
pub struct EPINT4_R(crate::FieldReader<bool, bool>);
impl EPINT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPINT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPINT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPINT5` reader - Pipe 5 Interrupt"]
pub struct EPINT5_R(crate::FieldReader<bool, bool>);
impl EPINT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPINT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPINT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPINT6` reader - Pipe 6 Interrupt"]
pub struct EPINT6_R(crate::FieldReader<bool, bool>);
impl EPINT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPINT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPINT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPINT7` reader - Pipe 7 Interrupt"]
pub struct EPINT7_R(crate::FieldReader<bool, bool>);
impl EPINT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPINT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPINT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Pipe 0 Interrupt"]
    #[inline(always)]
    pub fn epint0(&self) -> EPINT0_R {
        EPINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pipe 1 Interrupt"]
    #[inline(always)]
    pub fn epint1(&self) -> EPINT1_R {
        EPINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pipe 2 Interrupt"]
    #[inline(always)]
    pub fn epint2(&self) -> EPINT2_R {
        EPINT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pipe 3 Interrupt"]
    #[inline(always)]
    pub fn epint3(&self) -> EPINT3_R {
        EPINT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pipe 4 Interrupt"]
    #[inline(always)]
    pub fn epint4(&self) -> EPINT4_R {
        EPINT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pipe 5 Interrupt"]
    #[inline(always)]
    pub fn epint5(&self) -> EPINT5_R {
        EPINT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pipe 6 Interrupt"]
    #[inline(always)]
    pub fn epint6(&self) -> EPINT6_R {
        EPINT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pipe 7 Interrupt"]
    #[inline(always)]
    pub fn epint7(&self) -> EPINT7_R {
        EPINT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "HOST Pipe Interrupt Summary\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintsmry](index.html) module"]
pub struct PINTSMRY_SPEC;
impl crate::RegisterSpec for PINTSMRY_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pintsmry::R](R) reader structure"]
impl crate::Readable for PINTSMRY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PINTSMRY to value 0"]
impl crate::Resettable for PINTSMRY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
