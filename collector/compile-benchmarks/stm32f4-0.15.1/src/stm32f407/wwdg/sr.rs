#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Early wakeup interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWIF_A {
    #[doc = "0: The EWI Interrupt Service Routine has been serviced"]
    Finished = 0,
    #[doc = "1: The EWI Interrupt Service Routine has been triggered"]
    Pending = 1,
}
impl From<EWIF_A> for bool {
    #[inline(always)]
    fn from(variant: EWIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIF` reader - Early wakeup interrupt flag"]
pub type EWIF_R = crate::BitReader<EWIF_A>;
impl EWIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWIF_A {
        match self.bits {
            false => EWIF_A::Finished,
            true => EWIF_A::Pending,
        }
    }
    #[doc = "Checks if the value of the field is `Finished`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == EWIF_A::Finished
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == EWIF_A::Pending
    }
}
#[doc = "Early wakeup interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWIF_AW {
    #[doc = "0: The EWI Interrupt Service Routine has been serviced"]
    Finished = 0,
}
impl From<EWIF_AW> for bool {
    #[inline(always)]
    fn from(variant: EWIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIF` writer - Early wakeup interrupt flag"]
pub type EWIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, EWIF_AW, O>;
impl<'a, const O: u8> EWIF_W<'a, O> {
    #[doc = "The EWI Interrupt Service Routine has been serviced"]
    #[inline(always)]
    pub fn finished(self) -> &'a mut W {
        self.variant(EWIF_AW::Finished)
    }
}
impl R {
    #[doc = "Bit 0 - Early wakeup interrupt flag"]
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Early wakeup interrupt flag"]
    #[inline(always)]
    pub fn ewif(&mut self) -> EWIF_W<0> {
        EWIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
